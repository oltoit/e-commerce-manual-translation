use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(length(min = 1, max = 50))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}