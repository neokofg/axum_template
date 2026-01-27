use axum::{Router, routing::get};

use super::handlers;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/health/ready", get(handlers::readiness_check))
        .route("/health/live", get(handlers::liveness_check))
}
