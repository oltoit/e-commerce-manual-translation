use actix_web::HttpResponse;
use crate::errors::error_response_body::ErrorResponseBody;

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
    CategoriesAlreadyAssociated(String),
    CategoriesNotAssociated(String),
    NoPropertyError(String),
    EnvLoaderError(String),
    ClientError,
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

pub const TOKEN_PARSING_ERROR_MSG: &'static str = "error parsing the token";
pub const TOKEN_GENERATION_ERROR_MSG: &'static str = "error generating the token";
pub const DTO_NOT_VALID_ERROR_MSG: &'static str = "DTO was not valid";
pub const CATEGORY_NOT_FOUND_MSG: &'static str = "category not found";
pub const PRODUCT_NOT_FOUND_MSG: &'static str = "product not found";
pub const USER_NOT_FOUND_MSG: &'static str = "user not found";
pub const SUBCATEGORY_UPDATE_ERROR_MSG: &'static str = "error updating subcategory";


impl ErrorsEnum {
    pub fn get_response(&self, path: &str) -> HttpResponse {
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
            ErrorsEnum::CategoriesAlreadyAssociated(msg) =>
                HttpResponse::BadRequest().json(
                  ErrorResponseBody::bad_request(path, msg)
                ),
            ErrorsEnum::CategoriesNotAssociated(msg) =>
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
            ErrorsEnum::ClientError =>
                HttpResponse::InternalServerError().json(
                    ErrorResponseBody::internal_server_error(path, "error setting client")
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