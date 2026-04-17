use diesel::PgConnection;
use crate::dao::user_repository;
use crate::errors::error_enums::{ErrorsEnum};
use crate::security::jwt_handler::generate_token;
use crate::security::role::Role;

pub fn authenticate(connection: &mut PgConnection, username: &str, password: &str) -> Result<String, ErrorsEnum> {
    let user = match user_repository::get_by_username(connection, username) {
        Ok(user) => user,
        Err(_) => return Err(ErrorsEnum::WrongCredentials),
    };

    let username_matches = username.eq(&user.username);
    let password_matches = bcrypt::verify(password, &user.password).unwrap_or(false);

    if username_matches && password_matches {
        let user_role = Role::from_str(&user.role).expect("Role is not valid");

        match generate_token(user.id, user.username, user_role) {
            Ok(token) => Ok(token),
            Err(_) => Err(ErrorsEnum::TokenParsing("Error parsing the token".to_string())),
        }
    } else {
        Err(ErrorsEnum::WrongCredentials)
    }
}