use crate::errors::WrongCredentialsError::WrongCredentialsError;

// TODO: implement authentication
pub fn authenticate(username: &str, password: &str) -> Result<String, WrongCredentialsError> {
    Ok(String::from("token"))
}