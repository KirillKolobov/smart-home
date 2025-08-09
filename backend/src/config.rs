use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub db_host: String,
    pub db_name: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_pass: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().expect("Failed to load .env file");
        let port = env::var("PORT")
            .expect("PORT MUST BE DEFINED!")
            .parse::<u16>()
            .expect("PORT must be a valid port number");
        let db_host = env::var("DB_HOST").expect("DB_HOST must be defined in .env");
        let db_port = env::var("DB_PORT")
            .expect("DB_PORT must be defined in .env")
            .parse::<u16>()
            .expect("DB_PORT must be a valid port number");
        let db_name = env::var("DB_NAME").expect("DB_NAME must be defined in .env");
        let db_user = env::var("DB_USER").expect("DB_USER must be defined in .env");
        let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD must be defined in .env");

        Self {
            port,
            db_host,
            db_name,
            db_port,
            db_user,
            db_pass,
        }
    }
}
