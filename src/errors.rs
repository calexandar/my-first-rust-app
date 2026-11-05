use std::error::Error;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    BadRequest(&'static str),
    NotFound,
    Internal(Box<dyn Error + Send + Sync>),
}

impl AppError {
    pub fn internal(error: impl Error + Send + Sync + 'static) -> Self {
        Self::Internal(Box::new(error))
    }
}

impl From<JsonRejection> for AppError {
    fn from(_: JsonRejection) -> Self {
        Self::BadRequest("Invalid request body.")
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            Self::Internal(error) => {
                tracing::error!(%error, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;

    use super::*;

    async fn body(response: Response) -> String {
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("a response body");

        String::from_utf8(bytes.to_vec()).expect("a UTF-8 body")
    }

    #[tokio::test]
    async fn bad_requests_use_the_generic_message() {
        let response = AppError::BadRequest("Invalid request body.").into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(body(response).await, "Invalid request body.");
    }

    #[tokio::test]
    async fn missing_resources_are_404s() {
        let response = AppError::NotFound.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(body(response).await, "Not Found");
    }

    #[tokio::test]
    async fn internal_errors_do_not_leak_details() {
        let response =
            AppError::internal(std::io::Error::other("database password leaked")).into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = body(response).await;
        assert_eq!(body, "Internal Server Error");
        assert!(!body.contains("password"));
    }
}
