use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use inertia::{Inertia, ValidationErrors, props};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::{AdminUser, hash_password_async};
use crate::errors::AppError;
use crate::http::into_http_response;
use crate::validation::is_email;

const PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct AdminUserRow {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct IndexQuery {
    #[serde(default)]
    page: Option<u32>,
    #[serde(default)]
    q: Option<String>,
    #[serde(default)]
    role: Option<String>,
}

#[derive(Debug, Serialize)]
struct Filters {
    q: String,
    role: &'static str,
}

#[derive(Debug, Serialize)]
struct Pagination {
    current_page: u32,
    last_page: u32,
    per_page: u32,
    total: i64,
    from: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UserForm {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    password_confirmation: String,
    #[serde(default)]
    is_admin: Option<bool>,
}

struct ValidUserForm {
    name: String,
    email: String,
    password: Option<String>,
    is_admin: Option<bool>,
}

pub async fn index(
    inertia: Inertia,
    State(pool): State<PgPool>,
    _admin: AdminUser,
    Query(query): Query<IndexQuery>,
) -> Result<Response, AppError> {
    let requested_page = query.page.unwrap_or(1).max(1);
    let q = query.q.unwrap_or_default().trim().to_owned();
    let role = match query.role.as_deref() {
        Some("admin") => Some(true),
        Some("member") => Some(false),
        _ => None,
    };
    let search = if q.is_empty() { None } else { Some(q.as_str()) };

    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM users
        WHERE ($1::text IS NULL OR name ILIKE '%' || $1 || '%' OR email ILIKE '%' || $1 || '%')
          AND ($2::boolean IS NULL OR is_admin = $2)
        "#,
    )
    .bind(search)
    .bind(role)
    .fetch_one(&pool)
    .await
    .map_err(AppError::internal)?;

    let last_page = if total == 0 {
        1
    } else {
        ((total + PER_PAGE - 1) / PER_PAGE) as u32
    };
    let page = requested_page.min(last_page);
    let offset = i64::from(page - 1) * PER_PAGE;

    let users = sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT id, name, email, is_admin,
               to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        FROM users
        WHERE ($1::text IS NULL OR name ILIKE '%' || $1 || '%' OR email ILIKE '%' || $1 || '%')
          AND ($2::boolean IS NULL OR is_admin = $2)
        ORDER BY created_at DESC, id DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(search)
    .bind(role)
    .bind(PER_PAGE)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(AppError::internal)?;

    let from = (!users.is_empty()).then_some(offset + 1);
    let to = (!users.is_empty()).then_some(offset + users.len() as i64);
    let filters = Filters {
        q,
        role: match role {
            Some(true) => "admin",
            Some(false) => "member",
            None => "all",
        },
    };
    let pagination = Pagination {
        current_page: page,
        last_page,
        per_page: PER_PAGE as u32,
        total,
        from,
        to,
    };

    Ok(inertia
        .render(
            "Admin/Users/Index",
            props! {
                users => users,
                filters => filters,
                pagination => pagination,
            },
        )
        .encrypt_history(true)
        .into_response())
}

pub async fn create_form(inertia: Inertia, _admin: AdminUser) -> Response {
    inertia
        .render("Admin/Users/Create", props! {})
        .encrypt_history(true)
        .into_response()
}

pub async fn store(
    inertia: Inertia,
    State(pool): State<PgPool>,
    _admin: AdminUser,
    payload: Result<Json<UserForm>, JsonRejection>,
) -> Result<Response, AppError> {
    let form = match validate_user_form(payload?.0, true) {
        Ok(form) => form,
        Err(errors) => return Ok(validation_response(inertia, errors)),
    };

    let password = form.password.ok_or_else(|| {
        AppError::internal(std::io::Error::other(
            "validated create form was missing a password",
        ))
    })?;
    let password_hash = hash_password_async(password).await?;

    let result = sqlx::query_as::<_, AdminUserRow>(
        r#"
        INSERT INTO users (name, email, password_hash, is_admin)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, is_admin,
                  to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        "#,
    )
    .bind(&form.name)
    .bind(&form.email)
    .bind(password_hash)
    .bind(form.is_admin.unwrap_or(false))
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => {
            inertia.flash("success", format!("{} was created.", user.name));
            Ok(into_http_response(inertia::redirect(&format!(
                "/admin/users/{}",
                user.id
            ))))
        }
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            let mut errors = ValidationErrors::new();
            errors.add("email", "An account with this email already exists.");
            Ok(validation_response(inertia, errors))
        }
        Err(error) => Err(AppError::internal(error)),
    }
}

pub async fn show(
    inertia: Inertia,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    _admin: AdminUser,
) -> Result<Response, AppError> {
    let user = find_user_row(&pool, id).await?.ok_or(AppError::NotFound)?;

    Ok(inertia
        .render("Admin/Users/Show", props! { user => user })
        .encrypt_history(true)
        .into_response())
}

pub async fn edit(
    inertia: Inertia,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    admin: AdminUser,
) -> Result<Response, AppError> {
    let user = find_user_row(&pool, id).await?.ok_or(AppError::NotFound)?;
    let is_self = user.id == admin.0.id;

    Ok(inertia
        .render(
            "Admin/Users/Edit",
            props! {
                user => user,
                isSelf => is_self,
            },
        )
        .encrypt_history(true)
        .into_response())
}

pub async fn update(
    inertia: Inertia,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    admin: AdminUser,
    payload: Result<Json<UserForm>, JsonRejection>,
) -> Result<Response, AppError> {
    let existing = find_user_row(&pool, id).await?.ok_or(AppError::NotFound)?;
    let form = match validate_user_form(payload?.0, false) {
        Ok(form) => form,
        Err(errors) => return Ok(validation_response(inertia, errors)),
    };

    let is_admin = form.is_admin.unwrap_or(existing.is_admin);
    if id == admin.0.id && !is_admin {
        let mut errors = ValidationErrors::new();
        errors.add("is_admin", "You cannot remove your own admin access.");
        return Ok(validation_response(inertia, errors));
    }

    let password_hash = match form.password {
        Some(password) => Some(hash_password_async(password).await?),
        None => None,
    };

    let result = sqlx::query_as::<_, AdminUserRow>(
        r#"
        UPDATE users
        SET name = $2,
            email = $3,
            is_admin = $4,
            password_hash = COALESCE($5, password_hash)
        WHERE id = $1
        RETURNING id, name, email, is_admin,
                  to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        "#,
    )
    .bind(id)
    .bind(&form.name)
    .bind(&form.email)
    .bind(is_admin)
    .bind(password_hash)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(user)) => {
            inertia.flash("success", format!("{} was updated.", user.name));
            Ok(into_http_response(inertia::redirect(&format!(
                "/admin/users/{}",
                user.id
            ))))
        }
        Ok(None) => Err(AppError::NotFound),
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            let mut errors = ValidationErrors::new();
            errors.add("email", "An account with this email already exists.");
            Ok(validation_response(inertia, errors))
        }
        Err(error) => Err(AppError::internal(error)),
    }
}

pub async fn destroy(
    inertia: Inertia,
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    admin: AdminUser,
) -> Result<Response, AppError> {
    let user = find_user_row(&pool, id).await?.ok_or(AppError::NotFound)?;

    if id == admin.0.id {
        return Err(AppError::Forbidden);
    }

    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::internal)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    inertia.flash("success", format!("{} was deleted.", user.name));
    Ok(into_http_response(inertia::redirect("/admin/users")))
}

async fn find_user_row(pool: &PgPool, id: i64) -> Result<Option<AdminUserRow>, AppError> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT id, name, email, is_admin,
               to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::internal)
}

fn validation_response(inertia: Inertia, errors: ValidationErrors) -> Response {
    into_http_response(inertia.back_with_errors(errors))
}

fn validate_user_form(
    form: UserForm,
    require_password: bool,
) -> Result<ValidUserForm, ValidationErrors> {
    let name = form.name.trim().to_owned();
    let email = form.email.trim().to_lowercase();
    let mut errors = ValidationErrors::new();

    if name.is_empty() {
        errors.add("name", "Please enter a name.");
    } else if name.chars().count() < 2 {
        errors.add("name", "Your name must be at least 2 characters.");
    } else if name.chars().count() > 80 {
        errors.add("name", "Your name must be 80 characters or fewer.");
    }

    if email.is_empty() {
        errors.add("email", "Please enter an email address.");
    } else if email.chars().count() > 254 || !is_email(&email) {
        errors.add("email", "Please enter a valid email address.");
    }

    let password = if form.password.is_empty() {
        if require_password {
            errors.add("password", "Please choose a password.");
        }

        if !form.password_confirmation.is_empty() {
            errors.add(
                "password_confirmation",
                "The password confirmation does not match.",
            );
        }

        None
    } else {
        let length = form.password.chars().count();

        if length < 12 {
            errors.add("password", "Your password must be at least 12 characters.");
        } else if length > 128 {
            errors.add("password", "Your password must be 128 characters or fewer.");
        }

        if form.password_confirmation != form.password {
            errors.add(
                "password_confirmation",
                "The password confirmation does not match.",
            );
        }

        Some(form.password)
    };

    if errors.is_empty() {
        Ok(ValidUserForm {
            name,
            email,
            password,
            is_admin: form.is_admin,
        })
    } else {
        Err(errors)
    }
}
