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
    pub frontend_origin: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().expect("Failed to load .env file");
        envy::from_env::<Config>().expect("Failed to load config from environment variables")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_from_env() {
        // Set up mock environment variables
        env::set_var("PORT", "8080");
        env::set_var("DB_HOST", "localhost");
        env::set_var("DB_NAME", "test_db");
        env::set_var("DB_PORT", "5432");
        env::set_var("DB_USER", "test_user");
        env::set_var("DB_PASS", "test_pass");
        env::set_var(
            "JWT_SECRET",
            "supersecretjwtkeythatisatleast32characterslong",
        );
        env::set_var("JWT_EXPIRES_IN", "3600");
        env::set_var("FRONTEND_ORIGIN", "http://localhost:3000");

        let config = Config::from_env();

        assert_eq!(config.port, 8080);
        assert_eq!(config.db_host, "localhost");
        assert_eq!(config.db_name, "test_db");
        assert_eq!(config.db_port, 5432);
        assert_eq!(config.db_user, "test_user");
        assert_eq!(config.db_pass, "test_pass");
        assert_eq!(
            config.jwt_secret,
            "supersecretjwtkeythatisatleast32characterslong"
        );
        assert_eq!(config.jwt_expires_in, 3600);
        assert_eq!(config.frontend_origin, "http://localhost:3000");

        // Clean up environment variables
        env::remove_var("PORT");
        env::remove_var("DB_HOST");
        env::remove_var("DB_NAME");
        env::remove_var("DB_PORT");
        env::remove_var("DB_USER");
        env::remove_var("DB_PASS");
        env::remove_var("JWT_SECRET");
        env::remove_var("JWT_EXPIRES_IN");
        env::remove_var("FRONTEND_ORIGIN");
    }
}
