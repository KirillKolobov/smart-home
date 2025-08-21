use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub db_host: String,
    pub db_name: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: String,
    pub jwt_secret: String,
    pub jwt_expires_in: u64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().expect("Failed to load .env file");
        envy::from_env::<Config>().expect("Failed to load config from environment variables")
    }
}
