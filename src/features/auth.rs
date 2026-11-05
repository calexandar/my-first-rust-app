use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use inertia::{Inertia, ValidationErrors, props};
use serde::Deserialize;
use sqlx::PgPool;
use tower_sessions::Session;

use crate::auth::{
    AuthenticatedUser, MaybeAuthenticatedUser, USER_ID_KEY, create_user, find_user_for_login,
    hash_password_async, verify_password_async,
};
use crate::csrf;
use crate::errors::AppError;
use crate::http::into_http_response;
use crate::validation::is_email;

const LOGIN_ERROR: &str = "These credentials do not match our records.";

#[derive(Deserialize)]
pub struct LoginForm {
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    password_confirmation: String,
}

pub async fn login_form(inertia: Inertia, current: MaybeAuthenticatedUser) -> Response {
    if current.0.is_some() {
        return into_http_response(inertia::redirect("/dashboard"));
    }

    inertia.render("Auth/Login", props! {}).into_response()
}

pub async fn register_form(inertia: Inertia, current: MaybeAuthenticatedUser) -> Response {
    if current.0.is_some() {
        return into_http_response(inertia::redirect("/dashboard"));
    }

    inertia.render("Auth/Register", props! {}).into_response()
}

pub async fn login(
    inertia: Inertia,
    State(pool): State<PgPool>,
    session: Session,
    payload: Result<Json<LoginForm>, JsonRejection>,
) -> Result<Response, AppError> {
    let form = match validate_login(payload?.0) {
        Ok(form) => form,
        Err(errors) => return Ok(validation_response(inertia, errors)),
    };

    let user = find_user_for_login(&pool, &form.email)
        .await
        .map_err(AppError::internal)?;

    let valid = match &user {
        Some(user) => {
            verify_password_async(form.password.clone(), user.password_hash.clone()).await
        }
        None => {
            let _ = hash_password_async(form.password.clone()).await?;
            false
        }
    };

    let Some(user) = user.filter(|_| valid) else {
        let mut errors = ValidationErrors::new();
        errors.add("email", LOGIN_ERROR);
        return Ok(validation_response(inertia, errors));
    };

    session.cycle_id().await.map_err(AppError::internal)?;
    session
        .insert(USER_ID_KEY, user.id)
        .await
        .map_err(AppError::internal)?;
    csrf::rotate_token(&session).await?;

    inertia.flash("success", "Welcome back!");
    Ok(into_http_response(inertia::redirect("/dashboard")))
}

pub async fn register(
    inertia: Inertia,
    State(pool): State<PgPool>,
    session: Session,
    payload: Result<Json<RegisterForm>, JsonRejection>,
) -> Result<Response, AppError> {
    let form = match validate_register(payload?.0) {
        Ok(form) => form,
        Err(errors) => return Ok(validation_response(inertia, errors)),
    };

    let password_hash = hash_password_async(form.password).await?;

    match create_user(&pool, &form.name, &form.email, &password_hash).await {
        Ok(user) => {
            session.cycle_id().await.map_err(AppError::internal)?;
            session
                .insert(USER_ID_KEY, user.id)
                .await
                .map_err(AppError::internal)?;
            csrf::rotate_token(&session).await?;

            inertia.flash("success", "Your account has been created.");
            Ok(into_http_response(inertia::redirect("/dashboard")))
        }
        Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
            let mut errors = ValidationErrors::new();
            errors.add("email", "An account with this email already exists.");
            Ok(validation_response(inertia, errors))
        }
        Err(error) => Err(AppError::internal(error)),
    }
}

pub async fn logout(
    inertia: Inertia,
    session: Session,
    _user: AuthenticatedUser,
) -> Result<Response, AppError> {
    session.flush().await.map_err(AppError::internal)?;
    inertia.clear_history();
    inertia.flash("success", "You have been signed out.");

    Ok(into_http_response(inertia::redirect("/")))
}

fn validation_response(inertia: Inertia, errors: ValidationErrors) -> Response {
    into_http_response(inertia.back_with_errors(errors))
}

fn validate_login(form: LoginForm) -> Result<LoginForm, ValidationErrors> {
    let email = form.email.trim().to_lowercase();
    let mut errors = ValidationErrors::new();

    if email.is_empty() {
        errors.add("email", "Please enter your email address.");
    } else if email.chars().count() > 254 || !is_email(&email) {
        errors.add("email", "Please enter a valid email address.");
    }

    if form.password.is_empty() {
        errors.add("password", "Please enter your password.");
    }

    if errors.is_empty() {
        Ok(LoginForm {
            email,
            password: form.password,
        })
    } else {
        Err(errors)
    }
}

fn validate_register(form: RegisterForm) -> Result<RegisterForm, ValidationErrors> {
    let name = form.name.trim().to_owned();
    let email = form.email.trim().to_lowercase();
    let mut errors = ValidationErrors::new();

    if name.is_empty() {
        errors.add("name", "Please enter your name.");
    } else if name.chars().count() < 2 {
        errors.add("name", "Your name must be at least 2 characters.");
    } else if name.chars().count() > 80 {
        errors.add("name", "Your name must be 80 characters or fewer.");
    }

    if email.is_empty() {
        errors.add("email", "Please enter your email address.");
    } else if email.chars().count() > 254 || !is_email(&email) {
        errors.add("email", "Please enter a valid email address.");
    }

    let password_length = form.password.chars().count();
    if form.password.is_empty() {
        errors.add("password", "Please choose a password.");
    } else if password_length < 12 {
        errors.add("password", "Your password must be at least 12 characters.");
    } else if password_length > 128 {
        errors.add("password", "Your password must be 128 characters or fewer.");
    }

    if form.password_confirmation != form.password {
        errors.add(
            "password_confirmation",
            "The password confirmation does not match.",
        );
    }

    if errors.is_empty() {
        Ok(RegisterForm {
            name,
            email,
            password: form.password,
            password_confirmation: form.password_confirmation,
        })
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn login_form(email: &str, password: &str) -> LoginForm {
        LoginForm {
            email: email.to_owned(),
            password: password.to_owned(),
        }
    }

    fn register_form(
        name: &str,
        email: &str,
        password: &str,
        password_confirmation: &str,
    ) -> RegisterForm {
        RegisterForm {
            name: name.to_owned(),
            email: email.to_owned(),
            password: password.to_owned(),
            password_confirmation: password_confirmation.to_owned(),
        }
    }

    #[test]
    fn login_normalizes_the_email_and_keeps_the_password() {
        let form = validate_login(login_form("  USER@Example.COM  ", " password with spaces "))
            .expect("a valid login form");

        assert_eq!(form.email, "user@example.com");
        assert_eq!(form.password, " password with spaces ");
    }

    #[test]
    fn login_requires_an_email_and_password() {
        let errors = match validate_login(login_form("", "")) {
            Ok(_) => panic!("an incomplete login should not be valid"),
            Err(errors) => errors,
        };

        assert_eq!(
            errors.first("email"),
            Some("Please enter your email address.")
        );
        assert_eq!(
            errors.first("password"),
            Some("Please enter your password.")
        );
    }

    #[test]
    fn registration_requires_matching_strong_passwords() {
        let errors =
            match validate_register(register_form("A", "not-an-email", "short", "different")) {
                Ok(_) => panic!("an incomplete registration should not be valid"),
                Err(errors) => errors,
            };

        assert_eq!(
            errors.first("name"),
            Some("Your name must be at least 2 characters.")
        );
        assert_eq!(
            errors.first("email"),
            Some("Please enter a valid email address.")
        );
        assert_eq!(
            errors.first("password"),
            Some("Your password must be at least 12 characters.")
        );
        assert_eq!(
            errors.first("password_confirmation"),
            Some("The password confirmation does not match.")
        );
    }

    #[test]
    fn a_complete_registration_is_valid() {
        let form = validate_register(register_form(
            "  Taylor  ",
            " Taylor@Example.com ",
            "correct horse battery staple",
            "correct horse battery staple",
        ))
        .expect("a valid registration");

        assert_eq!(form.name, "Taylor");
        assert_eq!(form.email, "taylor@example.com");
    }
}
