use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePasswordDto {
    pub username: String,
    pub password: String,
}
