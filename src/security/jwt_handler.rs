use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::config::env_loader::LOADER;
use crate::errors::token_error::{TokenGenerationError, TokenParsingError};
use crate::security::role::Role;

const TOKEN_PREFIX: &'static str = "Bearer";
const TOKEN_EXPIRATION: u32 = 3_600_00;

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    sub: i64,
    username: String,
    roles: Role,
    iat: u64,
    exp: u64,
}

impl TokenClaims {
    pub fn new(id: i64, username: String, roles: Role) -> Result<Self, SystemTimeError> {
        let iat = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let exp = iat + TOKEN_EXPIRATION as u64;

        Ok (TokenClaims {
            sub: id,
            username,
            roles,
            iat,
            exp,
        })
    }
    pub fn get_id(&self) -> i64 { self.sub }
    pub fn get_username(&self) -> &str { &self.username }
    pub fn get_role(&self) -> &Role { &self.roles }
    pub fn get_issued_at(&self) -> u64 { self.iat }
    pub fn get_expiration(&self) -> u64 { self.exp }
}

pub fn generate_token(id: i64, user_name: String, role: Role) -> Result<String, TokenGenerationError> {
    // FIXME: remove unwrap
    let secret_key = LOADER.get().unwrap().get_token_secret_key();
    let claims = match TokenClaims::new(id, user_name, role) {
        Ok(claims) => claims,
        Err(_) => return Err(TokenGenerationError),
    };

     match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes())) {
        Ok(token) => Ok(format!("{} {}", TOKEN_PREFIX, token)),
        Err(_) => Err(TokenGenerationError),
     }
}

pub fn parse_token(token: &str) -> Result<TokenClaims, TokenParsingError> {
    // FIXME: remove unwrap
    let secret_key = LOADER.get().unwrap().get_token_secret_key();

    match decode::<TokenClaims>(token.as_bytes(), &DecodingKey::from_secret(secret_key.as_bytes()), &Validation::new(Algorithm::HS256)) {
        Ok(token) => Ok(token.claims),
        Err(_) => Err(TokenParsingError),
    }
}