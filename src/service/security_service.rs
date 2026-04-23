use diesel::PgConnection;
use validator::Validate;
use crate::api::dto::auth_dto::LoginRequest;
use crate::dao::user_repository;
use crate::errors::error_enum::{ErrorsEnum, DTO_NOT_VALID_ERROR_MSG, TOKEN_PARSING_ERROR_MSG};
use crate::security::jwt_handler::generate_token;
use crate::security::role::Role;

pub fn authenticate(connection: &mut PgConnection, login_request: LoginRequest) -> Result<String, ErrorsEnum> {
    if login_request.validate().is_err() { return Err(ErrorsEnum::DTONotValid(DTO_NOT_VALID_ERROR_MSG.to_string())); }

    let user = match user_repository::get_by_username(connection, &login_request.username) {
        Ok(user) => user,
        Err(_) => return Err(ErrorsEnum::WrongCredentials),
    };

    let username_matches = login_request.username.eq(&user.username);
    let password_matches = bcrypt::verify(&login_request.password, &user.password).unwrap_or(false);

    if username_matches && password_matches {
        let user_role = Role::from_str(&user.role).expect("Role is not valid");

        match generate_token(user.id, user.username, user_role) {
            Ok(token) => Ok(token),
            Err(_) => Err(ErrorsEnum::TokenError(TOKEN_PARSING_ERROR_MSG.to_string())),
        }
    } else {
        Err(ErrorsEnum::WrongCredentials)
    }
}