use axum::body::Body;
use axum::response::Response;

pub fn into_http_response(response: inertia::HttpResponse) -> Response {
    let (parts, body) = response.into_parts();
    Response::from_parts(parts, Body::from(body))
}
