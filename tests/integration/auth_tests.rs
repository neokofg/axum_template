use axum_test::TestServer;
use serde_json::json;

use crate::common::{TestApp, UserFixture};

#[tokio::test]
async fn test_register_success() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();
    let user = UserFixture::new();

    let response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": user.email,
            "password": user.password,
            "name": user.name
        }))
        .await;

    response.assert_status(axum::http::StatusCode::CREATED);

    let json: serde_json::Value = response.json();
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["tokens"]["access_token"].is_string());
    assert!(json["data"]["tokens"]["refresh_token"].is_string());
}

#[tokio::test]
async fn test_register_invalid_email() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();

    let response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": "invalid-email",
            "password": "password123",
            "name": "Test User"
        }))
        .await;

    response.assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();

    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({
            "email": "nonexistent@example.com",
            "password": "wrongpassword"
        }))
        .await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}
