use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{HeaderMap, Request, StatusCode, header, request::Builder};
use inertia::testing::AssertablePage;
use tower::ServiceExt;

use rust_inertia_app::config::AppConfig;

fn test_config() -> AppConfig {
    AppConfig {
        env: "test".to_owned(),
        host: "127.0.0.1".to_owned(),
        port: 3000,
        url: "http://127.0.0.1:3000".to_owned(),
    }
}

fn test_app() -> Router {
    rust_inertia_app::app_with(&test_config(), inertia::Config::new().version("test"))
}

struct TestResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: String,
}

impl TestResponse {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|value| value.to_str().ok())
    }

    fn page(&self) -> AssertablePage {
        AssertablePage::from_body(&self.body)
    }

    fn session_cookie(&self) -> String {
        self.header("set-cookie")
            .and_then(|cookie| cookie.split(';').next())
            .expect("a session cookie")
            .to_owned()
    }
}

async fn send(app: &Router, request: Request<Body>) -> TestResponse {
    let response = app
        .clone()
        .oneshot(request)
        .await
        .expect("the router should answer");
    let (parts, body) = response.into_parts();
    let body = to_bytes(body, usize::MAX).await.expect("a response body");

    TestResponse {
        status: parts.status,
        headers: parts.headers,
        body: String::from_utf8(body.to_vec()).expect("a UTF-8 response body"),
    }
}

fn request(method: &str, uri: &str) -> Builder {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::HOST, "localhost")
}

fn visit(method: &str, uri: &str) -> Builder {
    request(method, uri)
        .header("x-inertia", "true")
        .header("x-inertia-version", "test")
}

fn get(uri: &str) -> Request<Body> {
    request("GET", uri)
        .body(Body::empty())
        .expect("a valid request")
}

fn form(method: &str, body: &str) -> Request<Body> {
    visit(method, "/contact")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::REFERER, "/contact")
        .body(Body::from(body.to_owned()))
        .expect("a valid request")
}

fn valid_form() -> &'static str {
    r#"{"name":"Taylor","email":"taylor@example.com","message":"Hello from the contact form."}"#
}

#[tokio::test]
async fn home_is_an_inertia_page() {
    let response = send(&test_app(), visit("GET", "/").body(Body::empty()).unwrap()).await;

    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Home")
        .equals("message", "Hello from Rust, React, and Inertia.")
        .equals("appName", "Rust Inertia App")
        .equals("environment", "test");
}

#[tokio::test]
async fn about_is_an_inertia_page() {
    let response = send(
        &test_app(),
        visit("GET", "/about").body(Body::empty()).unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);
    response.page().component("About");
}

#[tokio::test]
async fn contact_is_an_inertia_page() {
    let response = send(
        &test_app(),
        visit("GET", "/contact").body(Body::empty()).unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Contact")
        .equals("headline", "Send us a message")
        .equals("errors", serde_json::json!({}));
}

#[tokio::test]
async fn first_visit_returns_html() {
    let response = send(&test_app(), get("/")).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.body.contains("<!DOCTYPE html>"));
    assert!(response.body.contains("Home"));
}

#[tokio::test]
async fn health_does_not_render_a_page() {
    let response = send(&test_app(), get("/health")).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body, "ok");
}

#[tokio::test]
async fn unknown_routes_use_the_application_error_response() {
    let response = send(&test_app(), get("/does-not-exist")).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.body, "Not Found");
}

#[tokio::test]
async fn malformed_json_uses_the_application_error_response() {
    let response = send(&test_app(), form("POST", "{not json")).await;

    assert_eq!(response.status, StatusCode::BAD_REQUEST);
    assert_eq!(response.body, "Invalid request body.");
}

#[tokio::test]
async fn invalid_contact_submission_redirects_with_field_errors() {
    let app = test_app();
    let response = send(
        &app,
        form(
            "POST",
            r#"{"name":"Al","email":"not-an-email","message":"short"}"#,
        ),
    )
    .await;

    assert_eq!(response.status, StatusCode::FOUND);
    assert_eq!(response.header("location"), Some("/contact"));
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", "/contact")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Contact")
        .equals("errors.name", "Your name must be at least 3 characters.")
        .equals("errors.email", "Please enter a valid email address.")
        .equals(
            "errors.message",
            "Your message must be at least 10 characters.",
        );
}

#[tokio::test]
async fn valid_contact_submission_redirects_and_flashes_once() {
    let app = test_app();
    let response = send(&app, form("POST", valid_form())).await;

    assert_eq!(response.status, StatusCode::FOUND);
    assert_eq!(response.header("location"), Some("/contact"));
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", "/contact")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    response
        .page()
        .component("Contact")
        .equals("errors", serde_json::json!({}))
        .flash("success", "Thanks! Your message was received.");

    let response = send(
        &app,
        visit("GET", "/contact")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert!(response.page().page().flash.is_empty());
}

#[tokio::test]
async fn mutations_redirect_with_the_protocol_status() {
    let app = test_app();

    for method in ["PUT", "PATCH"] {
        let response = send(&app, form(method, valid_form())).await;

        assert_eq!(response.status, StatusCode::SEE_OTHER, "{method}");
        assert_eq!(response.header("location"), Some("/contact"), "{method}");
    }

    let response = send(
        &app,
        visit("DELETE", "/contact")
            .header(header::REFERER, "/contact")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::SEE_OTHER);
    assert_eq!(response.header("location"), Some("/contact"));

    let response = send(&app, form("POST", valid_form())).await;

    assert_eq!(response.status, StatusCode::FOUND);
    assert_eq!(response.header("location"), Some("/contact"));
}
