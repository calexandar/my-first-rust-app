pub mod config;
pub mod features;
pub mod vite;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use inertia::axum::InertiaLayer;
use inertia::{Config, View, props};
use tower_http::services::ServeDir;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::config::AppConfig;
use crate::features::{about, home};
use crate::vite::Vite;

const BODY_LIMIT: usize = 16 * 1024;

pub fn app(config: &AppConfig) -> Router {
    app_with(config, inertia_config())
}

pub fn app_with(config: &AppConfig, inertia: Config) -> Router {
    Router::new()
        .route("/", get(home::index))
        .route("/about", get(about::index))
        .route("/health", get(health))
        .layer(InertiaLayer::new(inertia))
        .layer(
            SessionManagerLayer::new(MemoryStore::default()).with_secure(config.secure_cookies()),
        )
        .layer(DefaultBodyLimit::max(BODY_LIMIT))
        .nest_service("/build", ServeDir::new("public/build"))
}

pub fn inertia_config() -> Config {
    let vite = Vite::new();

    Config::new()
        .version_with({
            let vite = vite.clone();
            move || vite.version()
        })
        .root_view(move |view: &View<'_>| {
            format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    {assets}
    {head}
</head>
<body>
    {body}
</body>
</html>"#,
                assets = vite.tags(),
                head = view.head.join("\n    "),
                body = view.body,
            )
        })
        .share(|_request| {
            props! {
                appName => "Rust Inertia App",
            }
        })
}

async fn health() -> &'static str {
    "ok"
}
