use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::shared::entity::login_request::LoginRequest;

#[derive(Deserialize, Validate)]
pub struct LoginRequestDto {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(length(min = 1, max = 50))]
    pub password: String,
}

impl LoginRequestDto {
    pub fn to_login_request(&self) -> LoginRequest<'_> {
        LoginRequest {
            username: self.username.as_str(),
            password: self.password.as_str(),
        }
    }
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}