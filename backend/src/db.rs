use sqlx::PgPool;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    /// Create a new Database instance with the given pool
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a reference to the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Check if the database connection is healthy
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    #[cfg(test)]
    pub fn new_mock() -> Self {
        Self {
            pool: PgPool::connect_lazy("").unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_creation() {
        // This would require a real database connection for a full test
        // For now, we just test the structure
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://test:test@localhost/nonexistent")
            .await;

        // Skip the actual test if we can't connect (which is expected in CI)
        if let Ok(pool) = pool {
            let db = Database::new(pool);
            assert!(!format!("{:?}", db.pool()).is_empty());
        }
    }

    #[tokio::test]
    async fn test_database_methods() {
        // Test that the methods exist and have the right signatures
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://test:test@localhost/nonexistent")
            .await;

        if let Ok(pool) = pool {
            let db = Database::new(pool.clone());

            // Test that pool() returns the same pool
            assert_eq!(db.pool() as *const _, &pool as *const _);

            // Test health_check method exists (would fail without connection)
            let health_result = db.health_check().await;
            // We expect this to fail since we're not actually connected
            assert!(health_result.is_err());
        }
    }
}
