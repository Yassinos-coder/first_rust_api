use sqlx::PgPool;

use crate::features::users::interfaces::user::User;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        username: &str,
        password_hash: &str,
        full_name: &str,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password_hash, full_name)
             VALUES ($1, $2, $3)
             RETURNING id, username, password_hash, full_name, created_at",
        )
        .bind(username)
        .bind(password_hash)
        .bind(full_name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_password(
        &self,
        username: &str,
        new_password_hash: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET password_hash = $1
             WHERE username = $2
             RETURNING id, username, password_hash, full_name, created_at",
        )
        .bind(new_password_hash)
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }
}
