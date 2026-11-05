use argon2::{Argon2, password_hash::PasswordHasher};

use crate::error::AppError;
use crate::features::users::dto::register_dto::RegisterDto;
use crate::features::users::dto::update_password_dto::UpdatePasswordDto;
use crate::features::users::interfaces::user::User;
use crate::features::users::repositories::user_repository::UserRepository;
use crate::features::users::validators::credentials_validator::CredentialsValidator;
use crate::features::users::validators::register_validator::RegisterValidator;

#[derive(Clone)]
pub struct AccountService {
    users: UserRepository,
}

impl AccountService {
    pub fn new(users: UserRepository) -> Self {
        Self { users }
    }

    pub async fn register(&self, dto: RegisterDto) -> Result<User, AppError> {
        RegisterValidator::validate(&dto)?;

        let username = dto.username.trim().to_lowercase();
        let full_name = dto.full_name.trim().to_string();
        let password_hash = Self::hash_password(dto.password).await?;

        self.users
            .create(&username, &password_hash, &full_name)
            .await
            .map_err(|error| match error {
                sqlx::Error::Database(db_error) if db_error.is_unique_violation() => {
                    AppError::Conflict("username already taken".into())
                }
                other => AppError::Internal(other.to_string()),
            })
    }

    pub async fn update_password(&self, dto: UpdatePasswordDto) -> Result<User, AppError> {
        CredentialsValidator::validate(&dto.username, &dto.password)?;

        let username = dto.username.trim().to_lowercase();
        let password_hash = Self::hash_password(dto.password).await?;

        self.users
            .update_password(&username, &password_hash)
            .await
            .map_err(|error| AppError::Internal(error.to_string()))?
            .ok_or_else(|| AppError::NotFound("user not found".into()))
    }

    async fn hash_password(password: String) -> Result<String, AppError> {
        tokio::task::spawn_blocking(move || {
            Argon2::default()
                .hash_password(password.as_bytes())
                .map(|hash| hash.to_string())
        })
        .await
        .map_err(|error| AppError::Internal(error.to_string()))?
        .map_err(|error| AppError::Internal(error.to_string()))
    }
}
