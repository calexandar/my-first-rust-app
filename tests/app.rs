use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use inertia::testing::AssertablePage;
use tower::ServiceExt;

fn test_config() -> rust_inertia_app::config::AppConfig {
    rust_inertia_app::config::AppConfig {
        env: "test".to_owned(),
        host: "127.0.0.1".to_owned(),
        port: 3000,
        url: "http://127.0.0.1:3000".to_owned(),
    }
}

fn test_app() -> Router {
    rust_inertia_app::app_with(&test_config(), inertia::Config::new().version("test"))
}

fn visit(uri: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header(header::HOST, "localhost")
        .header("x-inertia", "true")
        .header("x-inertia-version", "test")
        .body(Body::empty())
        .expect("a valid request")
}

async fn send(app: &Router, request: Request<Body>) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(request)
        .await
        .expect("the router should answer");
    let status = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("a response body")
        .to_bytes();

    (
        status,
        String::from_utf8(body.to_vec()).expect("a UTF-8 response body"),
    )
}

#[tokio::test]
async fn home_is_an_inertia_page() {
    let (status, body) = send(&test_app(), visit("/")).await;

    assert_eq!(status, StatusCode::OK);
    AssertablePage::from_body(&body)
        .component("Home")
        .equals("message", "Hello from Rust, React, and Inertia.");
}

#[tokio::test]
async fn about_is_an_inertia_page() {
    let (status, body) = send(&test_app(), visit("/about")).await;

    assert_eq!(status, StatusCode::OK);
    AssertablePage::from_body(&body).component("About");
}

#[tokio::test]
async fn first_visit_returns_html() {
    let (status, body) = send(
        &rust_inertia_app::app(&test_config()),
        Request::builder()
            .uri("/")
            .header(header::HOST, "localhost")
            .body(Body::empty())
            .expect("a valid request"),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<!DOCTYPE html>"));
    assert!(body.contains("Home"));
}

#[tokio::test]
async fn health_does_not_render_a_page() {
    let (status, body) = send(
        &test_app(),
        Request::builder()
            .uri("/health")
            .header(header::HOST, "localhost")
            .body(Body::empty())
            .expect("a valid request"),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "ok");
}
