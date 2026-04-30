use actix_web::{HttpMessage, HttpRequest};
use crate::shared::auth::auth_user::AuthUser;
use crate::api::error::ServiceError;
use crate::shared::errors::error_enum::ErrorsEnum;

pub fn get_auth_user_from_request(req: &HttpRequest) -> Result<AuthUser, ServiceError> {
    let extensions = req.extensions();
    match extensions.get::<AuthUser>() {
        Some(user) => Ok(user.clone()),
        None => Err(ServiceError::new(req.match_info().as_str().to_string(), ErrorsEnum::Forbidden))
    }
}