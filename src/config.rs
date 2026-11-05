use std::env;

#[derive(Clone, Debug)]
pub struct Config {
  pub port: u16,
  pub api_prefix: String,
  pub database_url: String,
}

impl Config {
  pub fn from_env() -> Self {
    dotenvy::dotenv().ok();
    let port = env::var("PORT")
    .expect("PORT must be set")
    .parse::<u16>()
    .expect("PORT must be a valid number");
    let api_prefix = env::var("API_PREFIX").expect("API_PREFIX must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Self { port, api_prefix, database_url }
  }
}