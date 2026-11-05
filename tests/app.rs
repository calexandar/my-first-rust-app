use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{HeaderMap, Request, StatusCode, header, request::Builder};
use inertia::testing::AssertablePage;
use sqlx::PgPool;
use tower::ServiceExt;
use tower_sessions_sqlx_store::PostgresStore;

use rust_inertia_app::auth;
use rust_inertia_app::config::AppConfig;
use rust_inertia_app::db;

const PASSWORD: &str = "correct horse battery staple";
const LOGIN_ERROR: &str = "These credentials do not match our records.";

fn test_config() -> AppConfig {
    AppConfig {
        env: "test".to_owned(),
        host: "127.0.0.1".to_owned(),
        port: 3000,
        url: "http://127.0.0.1:3000".to_owned(),
        database_url: std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://rust_inertia_app:change-me-local@127.0.0.1:5432/rust_inertia_app_test"
                .to_owned()
        }),
    }
}

async fn test_pool() -> PgPool {
    let config = test_config();
    let pool = db::connect(&config.database_url)
        .await
        .expect("the test database should be available; create it or set TEST_DATABASE_URL");
    db::migrate(&pool)
        .await
        .expect("the test database migrations should apply");

    pool
}

async fn test_app_with_pool() -> (Router, PgPool) {
    let config = test_config();
    let pool = test_pool().await;
    let app = rust_inertia_app::app_with(
        &config,
        inertia::Config::new().version("test"),
        pool.clone(),
    );

    (app, pool)
}

async fn test_app() -> Router {
    test_app_with_pool().await.0
}

#[derive(Debug)]
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

    fn cookie(&self) -> Option<String> {
        self.header("set-cookie")
            .and_then(|cookie| cookie.split(';').next())
            .map(str::to_owned)
    }

    fn session_cookie(&self) -> String {
        self.cookie().expect("a session cookie")
    }
}

#[derive(Clone, Debug)]
struct TestSession {
    cookie: String,
    csrf: String,
}

struct TestAdmin {
    session: TestSession,
    email: String,
    id: i64,
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

fn get_with_session(session: &TestSession, uri: &str) -> Request<Body> {
    visit("GET", uri)
        .header(header::COOKIE, &session.cookie)
        .body(Body::empty())
        .expect("a valid request")
}

fn form_with_session(session: &TestSession, method: &str, uri: &str, body: &str) -> Request<Body> {
    form_with_session_from(session, method, uri, uri, body)
}

fn form_with_session_from(
    session: &TestSession,
    method: &str,
    uri: &str,
    referer: &str,
    body: &str,
) -> Request<Body> {
    request(method, uri)
        .header("x-inertia", "true")
        .header("x-inertia-version", "test")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::REFERER, referer)
        .header(header::COOKIE, &session.cookie)
        .header("x-csrf-token", &session.csrf)
        .body(Body::from(body.to_owned()))
        .expect("a valid request")
}

fn delete_with_session(session: &TestSession, uri: &str) -> Request<Body> {
    visit("DELETE", uri)
        .header(header::REFERER, uri)
        .header(header::COOKIE, &session.cookie)
        .header("x-csrf-token", &session.csrf)
        .body(Body::empty())
        .expect("a valid request")
}

fn valid_form() -> &'static str {
    r#"{"name":"Taylor","email":"taylor@example.com","message":"Hello from the contact form."}"#
}

fn register_payload(name: &str, email: &str) -> String {
    serde_json::json!({
        "name": name,
        "email": email,
        "password": PASSWORD,
        "password_confirmation": PASSWORD,
    })
    .to_string()
}

fn login_payload(email: &str, password: &str) -> String {
    serde_json::json!({
        "email": email,
        "password": password,
    })
    .to_string()
}

fn managed_user_payload(name: &str, email: &str, is_admin: bool) -> String {
    serde_json::json!({
        "name": name,
        "email": email,
        "password": PASSWORD,
        "password_confirmation": PASSWORD,
        "is_admin": is_admin,
    })
    .to_string()
}

fn managed_user_update_payload(name: &str, email: &str, is_admin: bool) -> String {
    serde_json::json!({
        "name": name,
        "email": email,
        "is_admin": is_admin,
    })
    .to_string()
}

fn unique_email(prefix: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("a valid clock")
        .as_nanos();

    format!("{prefix}-{nanos}-{count}@example.com")
}

async fn start_session(app: &Router) -> TestSession {
    let response = send(app, visit("GET", "/contact").body(Body::empty()).unwrap()).await;

    assert_eq!(response.status, StatusCode::OK);

    session_from_page(&response, "")
}

fn session_from_page(response: &TestResponse, fallback_cookie: &str) -> TestSession {
    let page = response.page();
    let csrf = page
        .prop("csrfToken")
        .and_then(|value| value.as_str())
        .expect("a CSRF token in the shared props")
        .to_owned();

    TestSession {
        cookie: response
            .cookie()
            .unwrap_or_else(|| fallback_cookie.to_owned()),
        csrf,
    }
}

async fn register_session(app: &Router, name: &str, email: &str) -> TestSession {
    let guest = start_session(app).await;
    let response = send(
        app,
        form_with_session(&guest, "POST", "/register", &register_payload(name, email)),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/dashboard");
    let cookie = response.session_cookie();

    let response = send(
        app,
        visit("GET", "/dashboard")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);

    session_from_page(&response, &cookie)
}

async fn admin_session(app: &Router, pool: &PgPool) -> TestAdmin {
    let email = unique_email("admin");
    let session = register_session(app, "Admin User", &email).await;

    sqlx::query("UPDATE users SET is_admin = TRUE WHERE email = $1")
        .bind(&email)
        .execute(pool)
        .await
        .expect("the user should be promoted");

    let id: i64 = sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
        .bind(&email)
        .fetch_one(pool)
        .await
        .expect("the promoted user should exist");

    TestAdmin { session, email, id }
}

fn assert_redirect(response: &TestResponse, status: StatusCode, location: &str) {
    assert_eq!(response.status, status);
    assert_eq!(response.header("location"), Some(location));
}

#[tokio::test]
async fn pages_share_a_csrf_token_and_the_guest_auth_state() {
    let response = send(
        &test_app().await,
        visit("GET", "/login").body(Body::empty()).unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);

    let page = response.page();
    page.component("Auth/Login")
        .equals("auth.user", serde_json::Value::Null);

    assert!(
        page.prop("csrfToken")
            .and_then(|value| value.as_str())
            .is_some_and(|token| !token.is_empty()),
        "the login page should share a non-empty CSRF token"
    );
}

#[tokio::test]
async fn home_is_an_inertia_page() {
    let response = send(
        &test_app().await,
        visit("GET", "/").body(Body::empty()).unwrap(),
    )
    .await;

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
        &test_app().await,
        visit("GET", "/about").body(Body::empty()).unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);
    response.page().component("About");
}

#[tokio::test]
async fn contact_is_an_inertia_page() {
    let response = send(
        &test_app().await,
        visit("GET", "/contact").body(Body::empty()).unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Contact")
        .equals("headline", "Send us a message")
        .equals("errors", serde_json::json!({}))
        .has_count("messages", 0);
}

#[tokio::test]
async fn first_visit_returns_html() {
    let response = send(&test_app().await, get("/")).await;

    assert_eq!(response.status, StatusCode::OK);
    assert!(response.body.contains("<!DOCTYPE html>"));
    assert!(response.body.contains("Home"));
}

#[tokio::test]
async fn health_does_not_render_a_page() {
    let response = send(&test_app().await, get("/health")).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body, "ok");
}

#[tokio::test]
async fn unknown_routes_use_the_application_error_response() {
    let response = send(&test_app().await, get("/does-not-exist")).await;

    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.body, "Not Found");
}

#[tokio::test]
async fn malformed_json_uses_the_application_error_response() {
    let app = test_app().await;
    let session = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(&session, "POST", "/contact", "{not json"),
    )
    .await;

    assert_eq!(response.status, StatusCode::BAD_REQUEST);
    assert_eq!(response.body, "Invalid request body.");
}

#[tokio::test]
async fn invalid_contact_submission_redirects_with_field_errors() {
    let app = test_app().await;
    let session = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(
            &session,
            "POST",
            "/contact",
            r#"{"name":"Al","email":"not-an-email","message":"short"}"#,
        ),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/contact");
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
        )
        .has_count("messages", 0);
}

#[tokio::test]
async fn valid_contact_submission_redirects_and_flashes_once() {
    let app = test_app().await;
    let session = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(&session, "POST", "/contact", valid_form()),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/contact");
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
        .has_count("messages", 1)
        .equals("messages.0.name", "Taylor")
        .equals("messages.0.email", "taylor@example.com")
        .equals("messages.0.message", "Hello from the contact form.")
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
async fn contact_messages_are_only_listed_for_the_submitting_session() {
    let app = test_app().await;
    let session = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(&session, "POST", "/contact", valid_form()),
    )
    .await;
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
        .has_count("messages", 1)
        .equals("messages.0.name", "Taylor");

    let response = send(&app, visit("GET", "/contact").body(Body::empty()).unwrap()).await;

    response.page().has_count("messages", 0);
}

#[tokio::test]
async fn contact_input_is_stored_as_data_not_executed_as_sql() {
    let app = test_app().await;
    let session = start_session(&app).await;
    let name = "Robert'); DROP TABLE contact_messages; --";
    let message = "This message contains '; DROP TABLE contact_messages; -- and is just data.";
    let payload = serde_json::json!({
        "name": name,
        "email": "sql@example.com",
        "message": message,
    })
    .to_string();

    let response = send(
        &app,
        form_with_session(&session, "POST", "/contact", &payload),
    )
    .await;

    assert_eq!(response.status, StatusCode::FOUND);
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
        .has_count("messages", 1)
        .equals("messages.0.name", name)
        .equals("messages.0.message", message);
}

#[tokio::test]
async fn database_failures_do_not_leak_connection_details() {
    use std::time::Duration;

    use sqlx::postgres::PgPoolOptions;

    let valid_pool = test_pool().await;
    let session_store: PostgresStore = db::session_store(&valid_pool);
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(250))
        .connect_lazy("postgres://app_user:super-secret@127.0.0.1:1/app_db")
        .expect("a lazy pool");
    let app = rust_inertia_app::app_with_session_store(
        &test_config(),
        inertia::Config::new().version("test"),
        pool,
        session_store,
    );
    let session = start_session(&app).await;

    let response = send(
        &app,
        form_with_session(&session, "POST", "/contact", valid_form()),
    )
    .await;

    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.body, "Internal Server Error");
    assert!(!response.body.contains("super-secret"));
    assert!(!response.body.contains("app_user"));
}

#[tokio::test]
async fn mutations_redirect_with_the_protocol_status() {
    let app = test_app().await;
    let session = start_session(&app).await;

    for method in ["PUT", "PATCH"] {
        let response = send(
            &app,
            form_with_session(&session, method, "/contact", valid_form()),
        )
        .await;

        assert_eq!(response.status, StatusCode::SEE_OTHER, "{method}");
        assert_eq!(response.header("location"), Some("/contact"), "{method}");
    }

    let response = send(&app, delete_with_session(&session, "/contact")).await;

    assert_eq!(response.status, StatusCode::SEE_OTHER);
    assert_eq!(response.header("location"), Some("/contact"));

    let response = send(
        &app,
        form_with_session(&session, "POST", "/contact", valid_form()),
    )
    .await;

    assert_eq!(response.status, StatusCode::FOUND);
    assert_eq!(response.header("location"), Some("/contact"));
}

#[tokio::test]
async fn csrf_protects_mutating_requests() {
    let app = test_app().await;
    let session = start_session(&app).await;

    let response = send(
        &app,
        request("POST", "/contact")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::REFERER, "/contact")
            .header(header::COOKIE, &session.cookie)
            .body(Body::from(valid_form()))
            .unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::FORBIDDEN);
    assert_eq!(response.body, "Invalid CSRF token.");

    let response = send(
        &app,
        request("POST", "/login")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(login_payload("nobody@example.com", PASSWORD)))
            .unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::FORBIDDEN);
    assert!(
        response.cookie().is_none(),
        "a rejected CSRF request without a session should not create one"
    );
}

#[tokio::test]
async fn registration_logs_the_user_in_and_rotates_the_session() {
    let app = test_app().await;
    let guest = start_session(&app).await;
    let email = unique_email("register");

    let response = send(
        &app,
        form_with_session(
            &guest,
            "POST",
            "/register",
            &register_payload("Taylor", &email),
        ),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/dashboard");
    let cookie = response.session_cookie();
    assert_ne!(cookie, guest.cookie, "the session ID should rotate");

    let old_session_response = send(
        &app,
        visit("GET", "/dashboard")
            .header(header::COOKIE, &guest.cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_redirect(&old_session_response, StatusCode::SEE_OTHER, "/login");

    let response = send(
        &app,
        visit("GET", "/dashboard")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);

    let page = response.page();
    page.component("Dashboard")
        .equals("auth.user.name", "Taylor")
        .equals("auth.user.is_admin", false)
        .flash("success", "Your account has been created.");

    assert!(page.page().encrypt_history);
}

#[tokio::test]
async fn duplicate_registration_is_rejected() {
    let app = test_app().await;
    let email = unique_email("duplicate");

    let first = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(
            &first,
            "POST",
            "/register",
            &register_payload("Taylor", &email),
        ),
    )
    .await;
    assert_redirect(&response, StatusCode::FOUND, "/dashboard");

    let second = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(
            &second,
            "POST",
            "/register",
            &register_payload("Taylor Again", &email),
        ),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/register");
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", "/register")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    response
        .page()
        .equals("errors.email", "An account with this email already exists.");
}

#[tokio::test]
async fn login_rejects_bad_credentials_without_revealing_the_account() {
    let (app, pool) = test_app_with_pool().await;
    let email = unique_email("login");
    let password_hash = auth::hash_password_async(PASSWORD.to_owned())
        .await
        .expect("a password hash");
    auth::create_user(&pool, "Login User", &email, &password_hash)
        .await
        .expect("a test user");

    let unknown_email = unique_email("unknown");
    let unknown = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(
            &unknown,
            "POST",
            "/login",
            &login_payload(&unknown_email, PASSWORD),
        ),
    )
    .await;
    assert_redirect(&response, StatusCode::FOUND, "/login");
    let unknown_cookie = response.session_cookie();

    let wrong_password = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(
            &wrong_password,
            "POST",
            "/login",
            &login_payload(&email, "the wrong password"),
        ),
    )
    .await;
    assert_redirect(&response, StatusCode::FOUND, "/login");
    let wrong_cookie = response.session_cookie();

    let unknown_page = send(
        &app,
        visit("GET", "/login")
            .header(header::COOKIE, &unknown_cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    let wrong_page = send(
        &app,
        visit("GET", "/login")
            .header(header::COOKIE, &wrong_cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(
        unknown_page.page().prop("errors.email"),
        wrong_page.page().prop("errors.email")
    );
    assert_eq!(
        unknown_page
            .page()
            .prop("errors.email")
            .and_then(|value| value.as_str()),
        Some(LOGIN_ERROR)
    );
    unknown_page
        .page()
        .equals("auth.user", serde_json::Value::Null);
}

#[tokio::test]
async fn login_rotates_the_session_and_logout_invalidates_it() {
    let (app, pool) = test_app_with_pool().await;
    let email = unique_email("logout");
    let password_hash = auth::hash_password_async(PASSWORD.to_owned())
        .await
        .expect("a password hash");
    auth::create_user(&pool, "Logout User", &email, &password_hash)
        .await
        .expect("a test user");

    let guest = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(&guest, "POST", "/login", &login_payload(&email, PASSWORD)),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/dashboard");
    let logged_in_cookie = response.session_cookie();
    assert_ne!(logged_in_cookie, guest.cookie);

    let response = send(
        &app,
        visit("GET", "/dashboard")
            .header(header::COOKIE, &logged_in_cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status, StatusCode::OK);
    let session = session_from_page(&response, &logged_in_cookie);

    let response = send(&app, form_with_session(&session, "POST", "/logout", "{}")).await;

    assert_redirect(&response, StatusCode::FOUND, "/");
    let signed_out_cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", "/dashboard")
            .header(header::COOKIE, &logged_in_cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_redirect(&response, StatusCode::SEE_OTHER, "/login");

    let response = send(
        &app,
        visit("GET", "/")
            .header(header::COOKIE, &signed_out_cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(response.status, StatusCode::OK);
    let page = response.page();
    page.component("Home")
        .equals("auth.user", serde_json::Value::Null)
        .flash("success", "You have been signed out.");
    assert!(page.page().clear_history);
}

#[tokio::test]
async fn guests_and_non_admins_cannot_access_the_admin_page() {
    let (app, pool) = test_app_with_pool().await;

    let guest = start_session(&app).await;
    let response = send(&app, get_with_session(&guest, "/admin/users")).await;
    assert_redirect(&response, StatusCode::SEE_OTHER, "/login");

    let email = unique_email("member");
    let member = start_session(&app).await;
    let response = send(
        &app,
        form_with_session(
            &member,
            "POST",
            "/register",
            &register_payload("Member", &email),
        ),
    )
    .await;
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", "/dashboard")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    let member_session = session_from_page(&response, &cookie);

    let response = send(&app, get_with_session(&member_session, "/admin/users")).await;
    assert_eq!(response.status, StatusCode::FORBIDDEN);
    assert_eq!(response.body, "Forbidden");

    sqlx::query("UPDATE users SET is_admin = TRUE WHERE email = $1")
        .bind(&email)
        .execute(&pool)
        .await
        .expect("the user should be promoted");

    let uri = format!("/admin/users?q={email}");
    let response = send(&app, get_with_session(&member_session, &uri)).await;
    assert_eq!(response.status, StatusCode::OK);

    let page = response.page();
    page.component("Admin/Users/Index").has("users");

    let users = page
        .prop("users")
        .and_then(|value| value.as_array())
        .expect("a users list");
    assert!(
        users
            .iter()
            .any(|user| user["email"].as_str() == Some(email.as_str())),
        "the promoted user should be listed"
    );
    assert!(page.page().encrypt_history);
}

#[tokio::test]
async fn admin_users_index_filters_and_paginates_without_sensitive_fields() {
    let (app, pool) = test_app_with_pool().await;
    let admin = admin_session(&app, &pool).await;
    let local = unique_email("list");
    let prefix = local.split('@').next().expect("an email local part");

    for index in 0..12 {
        let name = format!("List User {index}");
        let email = format!("{prefix}-{index}@example.com");

        sqlx::query(
            "INSERT INTO users (name, email, password_hash, is_admin) VALUES ($1, $2, $3, $4)",
        )
        .bind(name)
        .bind(email)
        .bind("unused")
        .bind(false)
        .execute(&pool)
        .await
        .expect("a listed user");
    }

    sqlx::query("INSERT INTO users (name, email, password_hash, is_admin) VALUES ($1, $2, $3, $4)")
        .bind("List Admin")
        .bind(format!("{prefix}-admin@example.com"))
        .bind("unused")
        .bind(true)
        .execute(&pool)
        .await
        .expect("a listed admin");

    let uri = format!("/admin/users?q={prefix}&page=1");
    let response = send(&app, get_with_session(&admin.session, &uri)).await;
    assert_eq!(response.status, StatusCode::OK);

    let page = response.page();
    page.component("Admin/Users/Index")
        .has_count("users", 10)
        .equals("pagination.total", 13)
        .equals("pagination.current_page", 1)
        .equals("pagination.last_page", 2)
        .equals("filters.q", prefix)
        .missing("users.0.password_hash");
    assert!(!response.body.contains("password_hash"));
    assert!(!response.body.contains("unused"));
    assert!(page.page().encrypt_history);

    let uri = format!("/admin/users?q={prefix}&page=2");
    let response = send(&app, get_with_session(&admin.session, &uri)).await;
    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Admin/Users/Index")
        .has_count("users", 3)
        .equals("pagination.current_page", 2);

    let uri = format!("/admin/users?q={prefix}&role=admin");
    let response = send(&app, get_with_session(&admin.session, &uri)).await;
    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Admin/Users/Index")
        .has_count("users", 1)
        .equals("users.0.is_admin", true)
        .equals("pagination.total", 1);

    let uri = format!("/admin/users?q={prefix}&role=member");
    let response = send(&app, get_with_session(&admin.session, &uri)).await;
    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Admin/Users/Index")
        .has_count("users", 10)
        .equals("pagination.total", 12)
        .equals("pagination.last_page", 2);
}

#[tokio::test]
async fn admin_can_create_and_show_a_user() {
    let (app, pool) = test_app_with_pool().await;
    let admin = admin_session(&app, &pool).await;

    let response = send(
        &app,
        get_with_session(&admin.session, "/admin/users/create"),
    )
    .await;
    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Admin/Users/Create")
        .missing("user");

    let email = unique_email("created");
    let response = send(
        &app,
        form_with_session_from(
            &admin.session,
            "POST",
            "/admin/users",
            "/admin/users/create",
            &managed_user_payload("Created User", &email, true),
        ),
    )
    .await;

    assert_eq!(response.status, StatusCode::FOUND);
    let location = response
        .header("location")
        .expect("a redirect location")
        .to_owned();
    assert!(
        location.starts_with("/admin/users/"),
        "expected the created user URL, got {location}"
    );
    let id = location
        .rsplit('/')
        .next()
        .and_then(|value| value.parse::<i64>().ok())
        .expect("a created user id");
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", &location)
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status, StatusCode::OK);

    let page = response.page();
    page.component("Admin/Users/Show")
        .equals("user.name", "Created User")
        .equals("user.email", email)
        .equals("user.is_admin", true)
        .missing("user.password_hash")
        .flash("success", "Created User was created.");
    assert!(!response.body.contains("password_hash"));
    assert!(page.page().encrypt_history);

    let password_hash: String = sqlx::query_scalar("SELECT password_hash FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("the created user's password hash");
    assert!(password_hash.starts_with("$argon2id$"));
}

#[tokio::test]
async fn admin_create_user_validation_rejects_bad_input_and_duplicate_email() {
    let (app, pool) = test_app_with_pool().await;
    let admin = admin_session(&app, &pool).await;

    let response = send(
        &app,
        form_with_session_from(
            &admin.session,
            "POST",
            "/admin/users",
            "/admin/users/create",
            r#"{"name":"A","email":"not-an-email","password":"short","password_confirmation":"different","is_admin":false}"#,
        ),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/admin/users/create");
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", "/admin/users/create")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Admin/Users/Create")
        .equals("errors.name", "Your name must be at least 2 characters.")
        .equals("errors.email", "Please enter a valid email address.")
        .equals(
            "errors.password",
            "Your password must be at least 12 characters.",
        )
        .equals(
            "errors.password_confirmation",
            "The password confirmation does not match.",
        );

    let response = send(
        &app,
        form_with_session_from(
            &admin.session,
            "POST",
            "/admin/users",
            "/admin/users/create",
            &managed_user_payload("Duplicate User", &admin.email, false),
        ),
    )
    .await;

    assert_redirect(&response, StatusCode::FOUND, "/admin/users/create");
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", "/admin/users/create")
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    response
        .page()
        .component("Admin/Users/Create")
        .equals("errors.email", "An account with this email already exists.");
}

#[tokio::test]
async fn admin_can_update_a_user_but_cannot_demote_self() {
    let (app, pool) = test_app_with_pool().await;
    let admin = admin_session(&app, &pool).await;
    let email = unique_email("target");
    let password_hash = auth::hash_password(PASSWORD).expect("a password hash");

    let id: i64 = sqlx::query_scalar(
        "INSERT INTO users (name, email, password_hash, is_admin) VALUES ($1, $2, $3, FALSE) RETURNING id",
    )
    .bind("Target User")
    .bind(&email)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await
    .expect("a target user");

    let uri = format!("/admin/users/{id}");
    let edit_uri = format!("{uri}/edit");
    let response = send(&app, get_with_session(&admin.session, &edit_uri)).await;
    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Admin/Users/Edit")
        .equals("user.name", "Target User")
        .equals("isSelf", false)
        .missing("user.password_hash");

    let updated_email = unique_email("updated");
    let response = send(
        &app,
        form_with_session_from(
            &admin.session,
            "PUT",
            &uri,
            &edit_uri,
            &managed_user_update_payload("Updated User", &updated_email, true),
        ),
    )
    .await;

    assert_redirect(&response, StatusCode::SEE_OTHER, &uri);
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", &uri)
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status, StatusCode::OK);
    response
        .page()
        .component("Admin/Users/Show")
        .equals("user.name", "Updated User")
        .equals("user.email", &updated_email)
        .equals("user.is_admin", true)
        .missing("user.password_hash")
        .flash("success", "Updated User was updated.");

    let unchanged_hash: String =
        sqlx::query_scalar("SELECT password_hash FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .expect("the unchanged password hash");
    assert_eq!(unchanged_hash, password_hash);

    let new_password = "an even longer password";
    let password_payload = serde_json::json!({
        "name": "Updated User",
        "email": updated_email,
        "password": new_password,
        "password_confirmation": new_password,
        "is_admin": true,
    })
    .to_string();
    let response = send(
        &app,
        form_with_session_from(&admin.session, "PUT", &uri, &edit_uri, &password_payload),
    )
    .await;
    assert_redirect(&response, StatusCode::SEE_OTHER, &uri);

    let changed_hash: String = sqlx::query_scalar("SELECT password_hash FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("the changed password hash");
    assert_ne!(changed_hash, password_hash);
    assert!(auth::verify_password(new_password, &changed_hash));

    let own_uri = format!("/admin/users/{}", admin.id);
    let own_edit_uri = format!("{own_uri}/edit");
    let response = send(
        &app,
        form_with_session_from(
            &admin.session,
            "PUT",
            &own_uri,
            &own_edit_uri,
            &managed_user_update_payload("Admin User", &admin.email, false),
        ),
    )
    .await;

    assert_redirect(&response, StatusCode::SEE_OTHER, &own_edit_uri);
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", &own_edit_uri)
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status, StatusCode::OK);
    response.page().component("Admin/Users/Edit").equals(
        "errors.is_admin",
        "You cannot remove your own admin access.",
    );

    let is_admin: bool = sqlx::query_scalar("SELECT is_admin FROM users WHERE id = $1")
        .bind(admin.id)
        .fetch_one(&pool)
        .await
        .expect("the admin's role");
    assert!(is_admin);
}

#[tokio::test]
async fn admin_can_delete_a_user_but_not_self_and_missing_users_are_not_found() {
    let (app, pool) = test_app_with_pool().await;
    let admin = admin_session(&app, &pool).await;
    let email = unique_email("delete");

    let id: i64 = sqlx::query_scalar(
        "INSERT INTO users (name, email, password_hash, is_admin) VALUES ($1, $2, $3, FALSE) RETURNING id",
    )
    .bind("Delete Me")
    .bind(&email)
    .bind("unused")
    .fetch_one(&pool)
    .await
    .expect("a deletable user");

    let uri = format!("/admin/users/{id}");
    let response = send(&app, delete_with_session(&admin.session, &uri)).await;
    assert_redirect(&response, StatusCode::SEE_OTHER, "/admin/users");
    let cookie = response.session_cookie();

    let response = send(
        &app,
        visit("GET", &uri)
            .header(header::COOKIE, &cookie)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(response.status, StatusCode::NOT_FOUND);

    let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .expect("the deleted user lookup");
    assert!(exists.is_none());

    let own_uri = format!("/admin/users/{}", admin.id);
    let response = send(&app, delete_with_session(&admin.session, &own_uri)).await;
    assert_eq!(response.status, StatusCode::FORBIDDEN);
    assert_eq!(response.body, "Forbidden");

    let missing = 999_999_999_i64;
    let missing_uri = format!("/admin/users/{missing}");
    let missing_edit_uri = format!("{missing_uri}/edit");

    let response = send(&app, get_with_session(&admin.session, &missing_uri)).await;
    assert_eq!(response.status, StatusCode::NOT_FOUND);

    let response = send(&app, get_with_session(&admin.session, &missing_edit_uri)).await;
    assert_eq!(response.status, StatusCode::NOT_FOUND);

    let response = send(
        &app,
        form_with_session_from(
            &admin.session,
            "PUT",
            &missing_uri,
            &missing_edit_uri,
            &managed_user_update_payload("Nobody", &unique_email("nobody"), false),
        ),
    )
    .await;
    assert_eq!(response.status, StatusCode::NOT_FOUND);

    let response = send(&app, delete_with_session(&admin.session, &missing_uri)).await;
    assert_eq!(response.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn admin_user_management_requires_an_administrator() {
    let (app, _pool) = test_app_with_pool().await;
    let guest = start_session(&app).await;

    for uri in [
        "/admin/users/create",
        "/admin/users/1",
        "/admin/users/1/edit",
    ] {
        let response = send(&app, get_with_session(&guest, uri)).await;
        assert_redirect(&response, StatusCode::SEE_OTHER, "/login");
    }

    let guest_payload = managed_user_payload("Nope", &unique_email("nope"), false);
    let response = send(
        &app,
        form_with_session_from(
            &guest,
            "POST",
            "/admin/users",
            "/admin/users/create",
            &guest_payload,
        ),
    )
    .await;
    assert_redirect(&response, StatusCode::SEE_OTHER, "/login");

    let member_email = unique_email("plain");
    let member = register_session(&app, "Plain Member", &member_email).await;

    for uri in [
        "/admin/users/create",
        "/admin/users/1",
        "/admin/users/1/edit",
    ] {
        let response = send(&app, get_with_session(&member, uri)).await;
        assert_eq!(response.status, StatusCode::FORBIDDEN);
        assert_eq!(response.body, "Forbidden");
    }

    let response = send(
        &app,
        form_with_session_from(
            &member,
            "POST",
            "/admin/users",
            "/admin/users/create",
            &guest_payload,
        ),
    )
    .await;
    assert_eq!(response.status, StatusCode::FORBIDDEN);

    let response = send(
        &app,
        form_with_session_from(
            &member,
            "PUT",
            "/admin/users/1",
            "/admin/users/1/edit",
            &managed_user_update_payload("Nope", &unique_email("nope"), false),
        ),
    )
    .await;
    assert_eq!(response.status, StatusCode::FORBIDDEN);

    let response = send(&app, delete_with_session(&member, "/admin/users/1")).await;
    assert_eq!(response.status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn authenticated_users_are_redirected_away_from_login_and_register() {
    let app = test_app().await;
    let guest = start_session(&app).await;
    let email = unique_email("redirect");
    let response = send(
        &app,
        form_with_session(
            &guest,
            "POST",
            "/register",
            &register_payload("Redirect User", &email),
        ),
    )
    .await;
    let cookie = response.session_cookie();

    for uri in ["/login", "/register"] {
        let response = send(
            &app,
            visit("GET", uri)
                .header(header::COOKIE, &cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await;

        assert_redirect(&response, StatusCode::FOUND, "/dashboard");
    }
}
