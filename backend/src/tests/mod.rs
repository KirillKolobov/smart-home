pub mod device_integration_tests;
pub mod integration_tests;

#[cfg(test)]
pub use test_utils::*;

#[cfg(test)]
mod test_utils {
    use crate::config::Config;
    use sqlx::PgPool;

    pub fn create_test_config() -> Config {
        Config {
            port: 3001, // Use different port for tests
            db_host: "localhost".to_string(),
            db_name: "smart_home_test".to_string(),
            db_port: 5432,
            db_user: "smart_home_user".to_string(),
            db_pass: "1234".to_string(),
            jwt_secret: "test_secret_key_that_is_long_enough_for_jwt".to_string(),
            jwt_expires_in: 3600,
        }
    }

    pub async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
        let config = create_test_config();
        let connection_string = format!(
            "postgres://{user}:{password}@{host}:{port}/{dbname}?sslmode=disable",
            user = config.db_user,
            password = config.db_pass,
            host = config.db_host,
            port = config.db_port,
            dbname = config.db_name
        );

        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
    }

    pub async fn setup_test_database() -> Result<PgPool, Box<dyn std::error::Error>> {
        let pool = create_test_pool().await?;

        // Run migrations on test database
        sqlx::migrate!("./migrations").run(&pool).await?;

        // Clean up any existing test data
        sqlx::query!("TRUNCATE TABLE devices RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;
        sqlx::query!("TRUNCATE TABLE rooms RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;
        sqlx::query!("TRUNCATE TABLE user_houses RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;
        sqlx::query!("TRUNCATE TABLE houses RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;
        sqlx::query!("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;

        // Close the old pool and create a new one to ensure fresh connections
        pool.close().await;
        let new_pool = create_test_pool().await?;

        Ok(new_pool)
    }
}
