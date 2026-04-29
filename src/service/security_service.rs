use diesel::PgConnection;
use crate::outbound::dao::user_repository;
use crate::shared::errors::error_enum::{ErrorsEnum, TOKEN_PARSING_ERROR_MSG};
use crate::shared::auth::jwt_handler::generate_token;
use crate::shared::auth::role::Role;
use crate::shared::entity::login_request::LoginRequest;

pub fn authenticate(connection: &mut PgConnection, login_request: LoginRequest) -> Result<String, ErrorsEnum> {
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