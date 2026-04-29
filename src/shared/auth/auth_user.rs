use actix_web::{HttpMessage, HttpRequest};
use crate::shared::errors::error_enum::ErrorsEnum;
use crate::shared::auth::jwt_handler::TokenClaims;
use crate::shared::auth::role::Role;

#[derive(Clone)]
pub struct AuthUser {
    pub id: i64,
    pub role: Role
}

impl AuthUser {
    pub fn get(req: &HttpRequest) -> Result<Self, ErrorsEnum> {
        let extensions = req.extensions();
        match extensions.get::<AuthUser>() {
            Some(user) => Ok(user.clone()),
            None => Err(ErrorsEnum::Forbidden)
        }
    }
}

impl AuthUser {
    pub fn new(id: i64, role: Role) -> Self {
        AuthUser { id, role }
    }
    pub fn from(claims: TokenClaims) -> Self {
        AuthUser { id: claims.get_id(), role: claims.get_role().clone() }
    }
    pub fn get_id(&self) -> i64 { self.id }
    pub fn get_role(&self) -> Role { self.role }
}