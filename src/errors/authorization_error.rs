use std::fmt::{Display, Formatter};

pub struct AuthorizationError;
impl Display for AuthorizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token was not provided or invalid.")
    }
}