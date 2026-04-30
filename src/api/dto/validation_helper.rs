use crate::api::error::ServiceError;
use crate::shared::errors::error_enum::{ErrorsEnum, DTO_NOT_VALID_ERROR_MSG};

pub fn validate_dto<'a, T: validator::Validate>(dto: &'a T, path: &str) -> Result<&'a T, ServiceError> {
    match dto.validate() {
        Ok(_) => Ok(dto),
        Err(_) => Err(ServiceError::new(path.to_string(), ErrorsEnum::DTONotValid(DTO_NOT_VALID_ERROR_MSG.to_string())))
    }
}