use std::fmt::Display;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use crate::shared::errors::error_enum::ErrorsEnum;

#[derive(Debug)]
pub struct ServiceError {
    pub path: String,
    pub error: ErrorsEnum
}

impl ServiceError {
    pub fn new(path: String, error: ErrorsEnum) -> Self {
        ServiceError { path, error }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        self.error.get_response(&self.path).status()
    }
    fn error_response(&self) -> actix_web::HttpResponse {
        self.error.get_response(&self.path)
    }
}