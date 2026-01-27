use axum_test::TestServer;
use serde_json::json;

use crate::common::{TestApp, UserFixture};

async fn create_authenticated_user(server: &TestServer) -> (String, serde_json::Value) {
    let user = UserFixture::new();

    let response = server
        .post("/api/v1/auth/register")
        .json(&json!({
            "email": user.email,
            "password": user.password,
            "name": user.name
        }))
        .await;

    let json: serde_json::Value = response.json();
    let token = json["data"]["tokens"]["access_token"]
        .as_str()
        .unwrap()
        .to_string();

    (token, json["data"]["user"].clone())
}

#[tokio::test]
async fn test_get_current_user() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();

    let (token, user_info) = create_authenticated_user(&server).await;

    let response = server
        .get("/api/v1/users/me")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        )
        .await;

    response.assert_status_ok();

    let json: serde_json::Value = response.json();
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["email"], user_info["email"]);
}

#[tokio::test]
async fn test_get_current_user_unauthorized() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();

    let response = server.get("/api/v1/users/me").await;

    response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_current_user() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();

    let (token, _) = create_authenticated_user(&server).await;

    let response = server
        .put("/api/v1/users/me")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        )
        .json(&json!({
            "name": "Updated Name"
        }))
        .await;

    response.assert_status_ok();

    let json: serde_json::Value = response.json();
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["name"], "Updated Name");
}
