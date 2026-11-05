use std::error::Error;

use rust_inertia_app::config::AppConfig;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_inertia_app=info,inertia=info".into()),
        )
        .init();

    let config = AppConfig::from_env()?;
    let listener = TcpListener::bind((config.host.as_str(), config.port)).await?;

    tracing::info!(
        "listening on http://{}:{} ({})",
        config.host,
        config.port,
        config.env
    );

    axum::serve(listener, rust_inertia_app::app(&config))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(%error, "failed to listen for the shutdown signal");
    }
}
