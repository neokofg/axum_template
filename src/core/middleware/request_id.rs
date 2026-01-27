use axum::{
    body::Body,
    http::{Request, header::HeaderName},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

pub async fn request_id_middleware(mut request: Request<Body>, next: Next) -> Response {
    let request_id = request
        .headers()
        .get(&X_REQUEST_ID)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    request
        .headers_mut()
        .insert(X_REQUEST_ID.clone(), request_id.parse().unwrap());

    let mut response = next.run(request).await;

    response
        .headers_mut()
        .insert(X_REQUEST_ID.clone(), request_id.parse().unwrap());

    response
}
