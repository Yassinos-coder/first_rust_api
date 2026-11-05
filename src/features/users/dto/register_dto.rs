use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterDto {
    pub username: String,
    pub password: String,
    pub full_name: String,
}