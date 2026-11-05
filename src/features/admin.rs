use axum::extract::State;
use inertia::{Inertia, props};
use sqlx::PgPool;

use crate::auth::{AdminUser, list_users};
use crate::errors::AppError;

pub async fn users(
    inertia: Inertia,
    State(pool): State<PgPool>,
    _admin: AdminUser,
) -> Result<inertia::Response, AppError> {
    let users = list_users(&pool).await.map_err(AppError::internal)?;

    Ok(inertia
        .render("Admin/Users", props! { users => users })
        .encrypt_history(true))
}
