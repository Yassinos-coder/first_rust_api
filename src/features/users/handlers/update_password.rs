use axum::{Json, extract::State};

use crate::error::AppError;
use crate::features::users::dto::update_password_dto::UpdatePasswordDto;
use crate::features::users::dto::user_response::UserResponse;
use crate::features::users::mappers::user_mapper::UserMapper;
use crate::state::AppState;

pub async fn update_password(
    State(state): State<AppState>,
    Json(dto): Json<UpdatePasswordDto>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.account_service.update_password(dto).await?;
    Ok(Json(UserMapper::to_response(user)))
}
