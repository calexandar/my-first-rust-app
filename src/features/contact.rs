use axum::Json;
use axum::body::Body;
use axum::extract::rejection::JsonRejection;
use axum::response::Response;
use inertia::{Inertia, ValidationErrors, props};
use serde::Deserialize;

use crate::errors::AppError;

const SUCCESS_MESSAGE: &str = "Thanks! Your message was received.";
const UPDATE_MESSAGE: &str = "Contact details updated.";

#[derive(Debug, Default, Deserialize)]
pub struct ContactForm {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    message: String,
}

pub async fn index(inertia: Inertia) -> inertia::Response {
    inertia.render(
        "Contact",
        props! {
            headline => "Send us a message",
            supportEmail => "hello@example.com",
        },
    )
}

pub async fn store(
    inertia: Inertia,
    payload: Result<Json<ContactForm>, JsonRejection>,
) -> Result<Response, AppError> {
    submit(inertia, payload?, SUCCESS_MESSAGE).await
}

pub async fn update(
    inertia: Inertia,
    payload: Result<Json<ContactForm>, JsonRejection>,
) -> Result<Response, AppError> {
    submit(inertia, payload?, UPDATE_MESSAGE).await
}

pub async fn patch(
    inertia: Inertia,
    payload: Result<Json<ContactForm>, JsonRejection>,
) -> Result<Response, AppError> {
    submit(inertia, payload?, UPDATE_MESSAGE).await
}

pub async fn destroy(inertia: Inertia) -> Response {
    inertia.flash("success", "Contact details removed.");
    into_http_response(inertia::redirect("/contact"))
}

async fn submit(
    inertia: Inertia,
    Json(form): Json<ContactForm>,
    success_message: &'static str,
) -> Result<Response, AppError> {
    let errors = validate(&form);

    if !errors.is_empty() {
        return Ok(into_http_response(inertia.back_with_errors(errors)));
    }

    deliver(&form).await?;
    inertia.flash("success", success_message);

    Ok(into_http_response(inertia::redirect("/contact")))
}

async fn deliver(_form: &ContactForm) -> Result<(), AppError> {
    Ok(())
}

fn validate(form: &ContactForm) -> ValidationErrors {
    let mut errors = ValidationErrors::new();

    let name = form.name.trim();
    if name.is_empty() {
        errors.add("name", "Please enter your name.");
    } else if name.chars().count() < 3 {
        errors.add("name", "Your name must be at least 3 characters.");
    } else if name.chars().count() > 80 {
        errors.add("name", "Your name must be 80 characters or fewer.");
    }

    let email = form.email.trim();
    if email.is_empty() {
        errors.add("email", "Please enter your email address.");
    } else if !is_email(email) {
        errors.add("email", "Please enter a valid email address.");
    }

    let message = form.message.trim();
    if message.is_empty() {
        errors.add("message", "Please enter a message.");
    } else if message.chars().count() < 10 {
        errors.add("message", "Your message must be at least 10 characters.");
    } else if message.chars().count() > 1000 {
        errors.add("message", "Your message must be 1000 characters or fewer.");
    }

    errors
}

fn is_email(value: &str) -> bool {
    if value.chars().any(char::is_whitespace) {
        return false;
    }

    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };

    !local.is_empty()
        && !domain.is_empty()
        && !domain.contains('@')
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

fn into_http_response(response: inertia::HttpResponse) -> Response {
    let (parts, body) = response.into_parts();
    Response::from_parts(parts, Body::from(body))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form(name: &str, email: &str, message: &str) -> ContactForm {
        ContactForm {
            name: name.to_owned(),
            email: email.to_owned(),
            message: message.to_owned(),
        }
    }

    #[test]
    fn a_complete_form_is_valid() {
        assert!(
            validate(&form(
                "Taylor",
                "taylor@example.com",
                "Hello from the form."
            ))
            .is_empty()
        );
    }

    #[test]
    fn missing_fields_have_errors() {
        let errors = validate(&form("", "", ""));

        assert_eq!(errors.first("name"), Some("Please enter your name."));
        assert_eq!(
            errors.first("email"),
            Some("Please enter your email address.")
        );
        assert_eq!(errors.first("message"), Some("Please enter a message."));
    }

    #[test]
    fn short_and_long_values_are_rejected() {
        let errors = validate(&form("Al", "not-an-email", "Too short"));

        assert_eq!(
            errors.first("name"),
            Some("Your name must be at least 3 characters.")
        );
        assert_eq!(
            errors.first("email"),
            Some("Please enter a valid email address.")
        );
        assert_eq!(
            errors.first("message"),
            Some("Your message must be at least 10 characters.")
        );
    }

    #[test]
    fn email_validation_rejects_obviously_invalid_addresses() {
        for email in [
            "",
            "plain",
            "@example.com",
            "a@b",
            "a b@example.com",
            "a@example.",
        ] {
            assert!(!is_email(email), "{email}");
        }

        for email in ["a@example.com", "first.last+tag@sub.example.com"] {
            assert!(is_email(email), "{email}");
        }
    }
}
