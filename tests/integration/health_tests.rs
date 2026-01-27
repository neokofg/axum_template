use axum_test::TestServer;

use crate::common::TestApp;

#[tokio::test]
async fn test_health_check() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();

    let response = server.get("/health").await;

    response.assert_status_ok();

    let json: serde_json::Value = response.json();
    assert_eq!(json["status"], "ok");
}

#[tokio::test]
async fn test_liveness_check() {
    let app = TestApp::new().await;
    let server = TestServer::new(app.router()).unwrap();

    let response = server.get("/health/live").await;

    response.assert_status_ok();

    let json: serde_json::Value = response.json();
    assert_eq!(json["status"], "ok");
}
