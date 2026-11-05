use axum::Json;
use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::response::Response;
use inertia::{Inertia, ValidationErrors, props};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::errors::AppError;
use crate::http::into_http_response;
use crate::validation::is_email;

const SUCCESS_MESSAGE: &str = "Thanks! Your message was received.";
const UPDATE_MESSAGE: &str = "Contact details updated.";
const DELETE_MESSAGE: &str = "Contact details removed.";

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ContactMessage {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct ContactForm {
    #[serde(default)]
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    message: String,
}

impl ContactForm {
    fn trimmed(&self) -> Self {
        Self {
            name: self.name.trim().to_owned(),
            email: self.email.trim().to_owned(),
            message: self.message.trim().to_owned(),
        }
    }
}

pub async fn index(
    inertia: Inertia,
    State(pool): State<PgPool>,
    session: Session,
) -> Result<inertia::Response, AppError> {
    let messages = match session.id() {
        Some(id) => list_for_session(&pool, &id.to_string())
            .await
            .map_err(AppError::internal)?,
        None => Vec::new(),
    };

    Ok(inertia.render(
        "Contact",
        props! {
            headline => "Send us a message",
            supportEmail => "hello@example.com",
            messages => messages,
        },
    ))
}

pub async fn store(
    inertia: Inertia,
    State(pool): State<PgPool>,
    session: Session,
    payload: Result<Json<ContactForm>, JsonRejection>,
) -> Result<Response, AppError> {
    let (inertia, form) = match validate_form(inertia, payload?.0) {
        Ok(values) => values,
        Err(response) => return Ok(*response),
    };

    let visitor_id = visitor_id(&session).await?;
    create_message(&pool, &visitor_id, &form)
        .await
        .map_err(AppError::internal)?;

    Ok(success(inertia, SUCCESS_MESSAGE))
}

pub async fn update(
    inertia: Inertia,
    payload: Result<Json<ContactForm>, JsonRejection>,
) -> Result<Response, AppError> {
    without_persistence(inertia, payload?, UPDATE_MESSAGE)
}

pub async fn patch(
    inertia: Inertia,
    payload: Result<Json<ContactForm>, JsonRejection>,
) -> Result<Response, AppError> {
    without_persistence(inertia, payload?, UPDATE_MESSAGE)
}

pub async fn destroy(inertia: Inertia) -> Response {
    success(inertia, DELETE_MESSAGE)
}

async fn visitor_id(session: &Session) -> Result<String, AppError> {
    if let Some(id) = session.id() {
        return Ok(id.to_string());
    }

    session
        .insert("contact_visitor", true)
        .await
        .map_err(AppError::internal)?;
    session.save().await.map_err(AppError::internal)?;

    session
        .id()
        .map(|id| id.to_string())
        .ok_or_else(|| AppError::internal(std::io::Error::other("session id missing after save")))
}

pub async fn create_message(
    pool: &PgPool,
    session_id: &str,
    form: &ContactForm,
) -> Result<ContactMessage, sqlx::Error> {
    sqlx::query_as::<_, ContactMessage>(
        r#"
        INSERT INTO contact_messages (session_id, name, email, message)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, message,
                  to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        "#,
    )
    .bind(session_id)
    .bind(&form.name)
    .bind(&form.email)
    .bind(&form.message)
    .fetch_one(pool)
    .await
}

pub async fn list_for_session(
    pool: &PgPool,
    session_id: &str,
) -> Result<Vec<ContactMessage>, sqlx::Error> {
    sqlx::query_as::<_, ContactMessage>(
        r#"
        SELECT id, name, email, message,
               to_char(created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        FROM contact_messages
        WHERE session_id = $1
        ORDER BY contact_messages.created_at DESC, contact_messages.id DESC
        LIMIT 20
        "#,
    )
    .bind(session_id)
    .fetch_all(pool)
    .await
}

fn validate_form(
    inertia: Inertia,
    form: ContactForm,
) -> Result<(Inertia, ContactForm), Box<Response>> {
    let form = form.trimmed();
    let errors = validate(&form);

    if errors.is_empty() {
        Ok((inertia, form))
    } else {
        Err(Box::new(into_http_response(
            inertia.back_with_errors(errors),
        )))
    }
}

fn without_persistence(
    inertia: Inertia,
    Json(form): Json<ContactForm>,
    success_message: &'static str,
) -> Result<Response, AppError> {
    let (inertia, _form) = match validate_form(inertia, form) {
        Ok(values) => values,
        Err(response) => return Ok(*response),
    };

    Ok(success(inertia, success_message))
}

fn success(inertia: Inertia, message: &'static str) -> Response {
    inertia.flash("success", message);
    into_http_response(inertia::redirect("/contact"))
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
    fn form_values_are_trimmed_before_persistence() {
        let form = form("  Taylor  ", " taylor@example.com ", " Hello there. ").trimmed();

        assert_eq!(form.name, "Taylor");
        assert_eq!(form.email, "taylor@example.com");
        assert_eq!(form.message, "Hello there.");
    }
}
