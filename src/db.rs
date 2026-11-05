use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};
use tower_sessions_sqlx_store::PostgresStore;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await
}

pub async fn migrate(pool: &PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sqlx::migrate!("./migrations").run(pool).await?;
    session_store(pool).migrate().await?;

    Ok(())
}

pub fn session_store(pool: &PgPool) -> PostgresStore {
    PostgresStore::new(pool.clone())
}
