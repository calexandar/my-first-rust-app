use std::convert::Infallible;

use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash};
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use inertia::Inertia;
use serde::Serialize;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::errors::AppError;

pub const USER_ID_KEY: &str = "user_id";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, sqlx::FromRow)]
pub struct UserSummary {
    pub id: i64,
    pub name: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone)]
pub struct CurrentUser(pub UserSummary);

#[derive(Debug, Serialize)]
pub struct AuthProps {
    pub user: Option<UserSummary>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct AdminUserRow {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: String,
}

#[derive(sqlx::FromRow)]
pub struct LoginUser {
    pub id: i64,
    pub password_hash: String,
}

pub async fn load_current_user(
    State(pool): State<PgPool>,
    session: Session,
    inertia: Inertia,
    mut request: Request,
    next: Next,
) -> Response {
    let user = match session.get::<i64>(USER_ID_KEY).await {
        Ok(Some(id)) => match find_user_by_id(&pool, id).await {
            Ok(user) => user,
            Err(error) => return AppError::internal(error).into_response(),
        },
        Ok(None) => None,
        Err(error) => return AppError::internal(error).into_response(),
    };

    if let Some(user) = &user {
        request.extensions_mut().insert(CurrentUser(user.clone()));
    }

    inertia.share("auth", AuthProps { user });

    next.run(request).await
}

pub async fn find_user_by_id(pool: &PgPool, id: i64) -> Result<Option<UserSummary>, sqlx::Error> {
    sqlx::query_as::<_, UserSummary>("SELECT id, name, is_admin FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_user_for_login(
    pool: &PgPool,
    email: &str,
) -> Result<Option<LoginUser>, sqlx::Error> {
    sqlx::query_as::<_, LoginUser>("SELECT id, password_hash FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
}

pub async fn create_user(
    pool: &PgPool,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<UserSummary, sqlx::Error> {
    sqlx::query_as::<_, UserSummary>(
        r#"
        INSERT INTO users (name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, name, is_admin
        "#,
    )
    .bind(name)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<AdminUserRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT id, name, email, is_admin,
               to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        FROM users
        ORDER BY created_at DESC, id DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|error| AppError::internal(std::io::Error::other(error.to_string())))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub async fn hash_password_async(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(AppError::internal)?
}

pub async fn verify_password_async(password: String, hash: String) -> bool {
    tokio::task::spawn_blocking(move || verify_password(&password, &hash))
        .await
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub UserSummary);

impl<S: Send + Sync> FromRequestParts<S> for AuthenticatedUser {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentUser>()
            .map(|user| Self(user.0.clone()))
            .ok_or_else(|| Redirect::to("/login").into_response())
    }
}

#[derive(Debug, Clone)]
pub struct MaybeAuthenticatedUser(pub Option<UserSummary>);

impl<S: Send + Sync> FromRequestParts<S> for MaybeAuthenticatedUser {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(
            parts
                .extensions
                .get::<CurrentUser>()
                .map(|user| user.0.clone()),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct AdminUser(pub UserSummary);

impl<S: Send + Sync> FromRequestParts<S> for AdminUser {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthenticatedUser(user) = AuthenticatedUser::from_request_parts(parts, state).await?;

        if !user.is_admin {
            return Err(AppError::Forbidden.into_response());
        }

        Ok(Self(user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passwords_hash_and_verify() {
        let hash = hash_password("correct horse battery staple").expect("a password hash");

        assert!(verify_password("correct horse battery staple", &hash));
        assert!(!verify_password("wrong password", &hash));
        assert!(!verify_password("anything", "not-a-valid-hash"));
    }
}
