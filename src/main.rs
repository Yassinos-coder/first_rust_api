mod config;
mod db;
mod error;
mod features;
mod state;

use axum::{Json, Router, routing::get};
use config::Config;
use features::users::repositories::user_repository::UserRepository;
use features::users::services::account_service::AccountService;
use serde_json::{Value, json};
use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let pool = db::pool::create_pool(&config).await;
    tracing::info!("database connected");

    let user_repository = UserRepository::new(pool.clone());
    let account_service = AccountService::new(user_repository);

    let state = AppState {
        db: pool,
        config: config.clone(),
        account_service,
    };

    let api_routes = Router::new()
        .route("/health", get(health))
        .nest("/users", features::users::routes::router());
    let app = Router::new()
        .nest(&config.api_prefix, api_routes)
        .with_state(state);

    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    tracing::info!("listening on {}", address);

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
