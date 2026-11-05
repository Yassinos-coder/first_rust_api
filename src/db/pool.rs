use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::Config;

pub async fn create_pool(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await
        .expect("failed to connect to the database")
}
