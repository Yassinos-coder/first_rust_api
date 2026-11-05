use sqlx::PgPool;

use crate::config::Config;
use crate::features::users::services::account_service::AccountService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
    pub account_service: AccountService,
}
