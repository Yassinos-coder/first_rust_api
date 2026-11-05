use crate::error::AppError;

pub struct CredentialsValidator;

impl CredentialsValidator {
    pub fn validate(username: &str, password: &str) -> Result<(), AppError> {
        let username = username.trim();

        if username.len() < 3 || username.len() > 30 {
            return Err(AppError::BadRequest(
                "username must be 3-30 characters".into(),
            ));
        }

        if !username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(AppError::BadRequest(
                "username can only contain letters, numbers and underscores".into(),
            ));
        }

        if password.len() < 8 || password.len() > 128 {
            return Err(AppError::BadRequest(
                "password must be 8-128 characters".into(),
            ));
        }

        Ok(())
    }
}
