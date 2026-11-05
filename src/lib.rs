pub mod auth;
pub mod config;
pub mod csrf;
pub mod db;
pub mod errors;
pub mod features;
pub mod http;
pub mod validation;
pub mod vite;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::{get, post};
use inertia::axum::InertiaLayer;
use inertia::{Config, View, props};
use sqlx::PgPool;
use time::Duration;
use tower_http::services::ServeDir;
use tower_sessions::cookie::SameSite;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use crate::config::AppConfig;
use crate::errors::AppError;
use crate::features::{about, admin, auth as auth_handlers, contact, dashboard, home};
use crate::vite::Vite;

const BODY_LIMIT: usize = 16 * 1024;

pub fn app(config: &AppConfig, pool: PgPool) -> Router {
    app_with(config, inertia_config(), pool)
}

pub fn app_with(config: &AppConfig, inertia: Config, pool: PgPool) -> Router {
    let session_store = db::session_store(&pool);
    app_with_session_store(config, inertia, pool, session_store)
}

pub fn app_with_session_store(
    config: &AppConfig,
    inertia: Config,
    pool: PgPool,
    session_store: PostgresStore,
) -> Router {
    let inertia = inertia.share({
        let environment = config.env.clone();
        move |_request| {
            props! {
                appName => "Rust Inertia App",
                environment => environment.clone(),
            }
        }
    });

    let pages = Router::new()
        .route("/", get(home::index))
        .route("/about", get(about::index))
        .route(
            "/contact",
            get(contact::index)
                .post(contact::store)
                .put(contact::update)
                .patch(contact::patch)
                .delete(contact::destroy),
        )
        .route(
            "/login",
            get(auth_handlers::login_form).post(auth_handlers::login),
        )
        .route(
            "/register",
            get(auth_handlers::register_form).post(auth_handlers::register),
        )
        .route("/logout", post(auth_handlers::logout))
        .route("/dashboard", get(dashboard::index))
        .route("/admin/users", get(admin::index).post(admin::store))
        .route("/admin/users/create", get(admin::create_form))
        .route(
            "/admin/users/{id}",
            get(admin::show)
                .put(admin::update)
                .patch(admin::update)
                .delete(admin::destroy),
        )
        .route("/admin/users/{id}/edit", get(admin::edit))
        .fallback(not_found)
        .layer(from_fn_with_state(pool.clone(), auth::load_current_user))
        .layer(from_fn(csrf::protect))
        .layer(InertiaLayer::new(inertia))
        .layer(
            SessionManagerLayer::new(session_store)
                .with_name("rust_inertia_session")
                .with_http_only(true)
                .with_same_site(SameSite::Strict)
                .with_secure(config.secure_cookies())
                .with_path("/")
                .with_expiry(Expiry::OnInactivity(Duration::days(14))),
        )
        .layer(DefaultBodyLimit::max(BODY_LIMIT));

    Router::<PgPool>::new()
        .merge(pages)
        .route("/health", get(health))
        .nest_service("/build", ServeDir::new("public/build"))
        .with_state(pool)
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
    <link rel="icon" href="data:,">
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
}

async fn health() -> &'static str {
    "ok"
}

async fn not_found() -> AppError {
    AppError::NotFound
}
