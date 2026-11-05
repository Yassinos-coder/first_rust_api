use axum::{Json, extract::State, http::StatusCode};

use crate::error::AppError;
use crate::features::users::dto::register_dto::RegisterDto;
use crate::features::users::dto::user_response::UserResponse;
use crate::features::users::mappers::user_mapper::UserMapper;
use crate::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(dto): Json<RegisterDto>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = state.account_service.register(dto).await?;
    let response = UserMapper::to_response(user);
    Ok((StatusCode::CREATED, Json(response)))
}
