use crate::features::users::dto::user_response::UserResponse;
use crate::features::users::interfaces::user::User;

pub struct UserMapper;

impl UserMapper {
    pub fn to_response(user: User) -> UserResponse {
        UserResponse {
            id: user.id,
            username: user.username,
            full_name: user.full_name,
            created_at: user.created_at,
        }
    }
}
