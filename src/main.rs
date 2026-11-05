use std::error::Error;

use rust_inertia_app::config::AppConfig;
use tokio::net::TcpListener;
use tower_sessions::ExpiredDeletion;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_inertia_app=info,inertia=info".into()),
        )
        .init();

    let config = AppConfig::from_env()?;
    let pool = rust_inertia_app::db::connect(&config.database_url).await?;
    rust_inertia_app::db::migrate(&pool).await?;

    let session_store = rust_inertia_app::db::session_store(&pool);
    let cleanup_store = session_store.clone();
    tokio::spawn(async move {
        if let Err(error) = cleanup_store
            .continuously_delete_expired(tokio::time::Duration::from_secs(3600))
            .await
        {
            tracing::error!(%error, "session cleanup stopped");
        }
    });

    let router = rust_inertia_app::app_with_session_store(
        &config,
        rust_inertia_app::inertia_config(),
        pool,
        session_store,
    );
    let listener = TcpListener::bind((config.host.as_str(), config.port)).await?;

    tracing::info!(
        "listening on http://{}:{} ({})",
        config.host,
        config.port,
        config.env
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "failed to listen for the shutdown signal");
    }
}
