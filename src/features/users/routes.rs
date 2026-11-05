use axum::{
    Router,
    routing::{patch, post},
};

use crate::features::users::handlers::register::register;
use crate::features::users::handlers::update_password::update_password;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/password", patch(update_password))
}
