pub struct WrongCredentialsError;
impl std::fmt::Display for WrongCredentialsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The specified credentials are wrong.")
    }
}