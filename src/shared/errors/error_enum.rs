use std::fmt::{Display, Formatter};
use actix_web::HttpResponse;
use log::{error, warn};
use crate::shared::errors::error_response_body::ErrorResponseBody;

pub enum ErrorsEnum {
    WrongCredentials,
    TokenError(String),
    NotFound(String),
    Forbidden,
    CreationError(String),
    UpdateError(String),
    DeletionError(String),
    DieselError(String),
    DTONotValid(String),
    JsonParsingError(String),
    CategoryAssociationError(String),
    NoPropertyError(String),
    EnvLoaderError(String),
    FixerApiError,
    WrongCurrency(String),
    ProductCategoryError(String),
    DatabaseError(String),
}

impl From<diesel::result::Error> for ErrorsEnum {
    fn from(e: diesel::result::Error) -> Self {
        ErrorsEnum::DieselError(e.to_string())
    }
}

impl Display for ErrorsEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorsEnum::WrongCredentials => write!(f, "Wrong credentials"),
            ErrorsEnum::TokenError(msg) => write!(f, "Token error: {}", msg),
            ErrorsEnum::NotFound(msg) => write!(f, "Not found: {}", msg),
            ErrorsEnum::Forbidden => write!(f, "Forbidden"),
            ErrorsEnum::CreationError(msg) => write!(f, "Creation error: {}", msg),
            ErrorsEnum::UpdateError(msg) => write!(f, "Update error: {}", msg),
            ErrorsEnum::DeletionError(msg) => write!(f, "Deletion error: {}", msg),
            ErrorsEnum::DieselError(msg) => write!(f, "Diesel error: {}", msg),
            ErrorsEnum::DTONotValid(msg) => write!(f, "DTO not valid: {}", msg),
            ErrorsEnum::JsonParsingError(msg) => write!(f, "JSON parsing error: {}", msg),
            ErrorsEnum::CategoryAssociationError(msg) => write!(f, "Category Association error: {}", msg),
            ErrorsEnum::NoPropertyError(msg) => write!(f, "Property error: {}", msg),
            ErrorsEnum::EnvLoaderError(msg) => write!(f, "Env loader error: {}", msg),
            ErrorsEnum::FixerApiError => write!(f, "Fixer API error"),
            ErrorsEnum::WrongCurrency(msg) => write!(f, "Wrong currency: {}", msg),
            ErrorsEnum::ProductCategoryError(msg) => write!(f, "Product Category error: {}", msg),
            ErrorsEnum::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

pub const TOKEN_PARSING_ERROR_MSG: &'static str = "error parsing the token";
pub const TOKEN_GENERATION_ERROR_MSG: &'static str = "error generating the token";
pub const DTO_NOT_VALID_ERROR_MSG: &'static str = "DTO was not valid";
pub const CATEGORY_NOT_FOUND_MSG: &'static str = "category not found";
pub const PRODUCT_NOT_FOUND_MSG: &'static str = "product not found";
pub const USER_NOT_FOUND_MSG: &'static str = "user not found";
pub const SUBCATEGORY_UPDATE_ERROR_MSG: &'static str = "error updating subcategory";

impl ErrorsEnum {
    pub fn get_response(&self, path: &str) -> HttpResponse {
        let response = self.match_self_to_http_response(path);

        if response.status().is_server_error() {
            error!("{}", self);
        } else {
            warn!("{}", self);
        }

        response
    }

    fn match_self_to_http_response(&self, path: &str) -> HttpResponse {
        match self {
            ErrorsEnum::WrongCredentials =>
                HttpResponse::Forbidden().json(
                    ErrorResponseBody::forbidden(path)
                ),
            ErrorsEnum::TokenError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
            ErrorsEnum::NotFound(msg) =>
                HttpResponse::NotFound().json(
                    ErrorResponseBody::not_found(path, msg)
                ),
            ErrorsEnum::Forbidden =>
                HttpResponse::Forbidden().json(
                    ErrorResponseBody::forbidden(path)
                ),
            ErrorsEnum::CreationError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
            ErrorsEnum::UpdateError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
            ErrorsEnum::DeletionError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
            ErrorsEnum::DieselError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
            ErrorsEnum::DTONotValid(msg) =>
                HttpResponse::BadRequest().json(
                    ErrorResponseBody::bad_request(path, msg)
                ),
            ErrorsEnum::JsonParsingError(msg) =>
                HttpResponse::BadRequest().json(
                    ErrorResponseBody::bad_request(path, msg)
                ),
            ErrorsEnum::CategoryAssociationError(msg) =>
                HttpResponse::BadRequest().json(
                    ErrorResponseBody::bad_request(path, msg)
                ),
            ErrorsEnum::NoPropertyError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
            ErrorsEnum::EnvLoaderError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
            ErrorsEnum::FixerApiError =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, "error with fixer api")
                ),
            ErrorsEnum::WrongCurrency(msg) =>
                HttpResponse::BadRequest().json(
                    ErrorResponseBody::bad_request(path, msg)
                ),
            ErrorsEnum::ProductCategoryError(msg) =>
                HttpResponse::BadRequest().json(
                    ErrorResponseBody::bad_request(path, msg)
                ),
            ErrorsEnum::DatabaseError(msg) =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, msg)
                ),
        }
    }
}