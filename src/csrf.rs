use axum::extract::Request;
use axum::http::{Method, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use inertia::Inertia;
use tower_sessions::Session;
use tower_sessions::session::Id;

use crate::errors::AppError;

pub const CSRF_SESSION_KEY: &str = "csrf_token";
const CSRF_HEADER: &str = "x-csrf-token";

pub async fn protect(session: Session, inertia: Inertia, request: Request, next: Next) -> Response {
    let existing = match session.get::<String>(CSRF_SESSION_KEY).await {
        Ok(token) => token.filter(|token| !token.is_empty()),
        Err(error) => return AppError::internal(error).into_response(),
    };

    let token = if is_safe(request.method()) {
        match existing {
            Some(token) => token,
            None => {
                let token = new_token();
                if let Err(error) = session.insert(CSRF_SESSION_KEY, &token).await {
                    return AppError::internal(error).into_response();
                }
                token
            }
        }
    } else {
        let Some(token) = existing else {
            return forbidden();
        };

        let provided = request
            .headers()
            .get(CSRF_HEADER)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();

        if !constant_time_eq(provided, &token) {
            return forbidden();
        }

        token
    };

    inertia.share("csrfToken", token);

    next.run(request).await
}

pub async fn rotate_token(session: &Session) -> Result<String, AppError> {
    let token = new_token();
    session
        .insert(CSRF_SESSION_KEY, &token)
        .await
        .map_err(AppError::internal)?;
    Ok(token)
}

fn new_token() -> String {
    Id::default().to_string()
}

fn is_safe(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::OPTIONS | Method::TRACE
    )
}

fn forbidden() -> Response {
    (StatusCode::FORBIDDEN, "Invalid CSRF token.").into_response()
}

fn constant_time_eq(provided: &str, expected: &str) -> bool {
    let provided = provided.as_bytes();
    let expected = expected.as_bytes();

    if provided.len() != expected.len() {
        return false;
    }

    let mut difference = 0u8;
    for (left, right) in provided.iter().zip(expected) {
        difference |= left ^ right;
    }

    difference == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_are_compared_without_early_exit() {
        assert!(constant_time_eq("abc", "abc"));
        assert!(!constant_time_eq("abc", "abd"));
        assert!(!constant_time_eq("abc", "ab"));
        assert!(!constant_time_eq("", "not-empty"));
    }

    #[test]
    fn only_safe_methods_are_exempt() {
        for method in [Method::GET, Method::HEAD, Method::OPTIONS, Method::TRACE] {
            assert!(is_safe(&method), "{method}");
        }

        for method in [Method::POST, Method::PUT, Method::PATCH, Method::DELETE] {
            assert!(!is_safe(&method), "{method}");
        }
    }
}
