use crate::error::AppError;
use crate::features::users::dto::register_dto::RegisterDto;
use crate::features::users::validators::credentials_validator::CredentialsValidator;

pub struct RegisterValidator;

impl RegisterValidator {
    pub fn validate(dto: &RegisterDto) -> Result<(), AppError> {
        CredentialsValidator::validate(&dto.username, &dto.password)?;

        if dto.full_name.trim().is_empty() {
            return Err(AppError::BadRequest("full name is required".into()));
        }

        Ok(())
    }
}
