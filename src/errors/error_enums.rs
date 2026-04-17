use actix_web::HttpResponse;
use crate::errors::error_response_body::ErrorResponseBody;

pub enum ErrorsEnum {
    WrongCredentials,
    TokenParsing(String),
    TokenGenerationError(String),
    NotFound(String),
    Forbidden,
    CreationError(String),
    UpdateError(String),
    DeletionError(String),
    DieselError(String),
}

impl From<diesel::result::Error> for ErrorsEnum {
    fn from(e: diesel::result::Error) -> Self {
        ErrorsEnum::DieselError(e.to_string())
    }
}

pub const TOKEN_PARSING_ERROR_MSG: &'static str = "Error parsing the token";
pub const TOKEN_GENERATION_ERROR_MSG: &'static str = "Error generating the token";

impl ErrorsEnum {
    pub fn get_response(&self, path: String) -> HttpResponse {
        match self {
            ErrorsEnum::WrongCredentials => HttpResponse::Forbidden().json(forbidden(path)),
            ErrorsEnum::TokenParsing(msg) => HttpResponse::InternalServerError().json(internal_server_error(path, msg.to_string())),
            ErrorsEnum::TokenGenerationError(msg) => HttpResponse::InternalServerError().json(internal_server_error(path, msg.to_string())),
            ErrorsEnum::NotFound(msg) => HttpResponse::NotFound().json(not_found(path, msg.to_string())),
            ErrorsEnum::Forbidden => HttpResponse::Unauthorized().json(forbidden(path)),
            ErrorsEnum::CreationError(msg) => HttpResponse::InternalServerError().json(internal_server_error(path, msg.to_string())),
            ErrorsEnum::UpdateError(msg) => HttpResponse::InternalServerError().json(internal_server_error(path, msg.to_string())),
            ErrorsEnum::DeletionError(msg) => HttpResponse::InternalServerError().json(internal_server_error(path, msg.to_string())),
            ErrorsEnum::DieselError(msg) => HttpResponse::InternalServerError().json(internal_server_error(path, msg.to_string())),
        }
    }
}

fn internal_server_error(path: String, msg: String) -> ErrorResponseBody {
    ErrorResponseBody::internal_server_error(path, msg)
}
fn not_found(path: String, msg: String) -> ErrorResponseBody {
    ErrorResponseBody::not_found(path, msg)
}

fn forbidden(path: String) -> ErrorResponseBody {
    ErrorResponseBody::forbidden(path)
}