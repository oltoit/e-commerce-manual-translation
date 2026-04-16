pub struct TokenGenerationError;
impl std::fmt::Display for TokenGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem generating the token.")
    }
}

pub struct TokenParsingError;
impl std::fmt::Display for TokenParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing the token.")
    }
}