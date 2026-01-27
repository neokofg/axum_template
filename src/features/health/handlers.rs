use axum::{Json, extract::State};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
pub struct ReadinessResponse {
    status: String,
    database: ServiceStatus,
    redis: ServiceStatus,
}

#[derive(Serialize)]
pub struct ServiceStatus {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn liveness_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn readiness_check(State(state): State<AppState>) -> Json<ReadinessResponse> {
    let db_status = check_database(&state).await;
    let redis_status = check_redis(&state).await;

    let overall_status = if db_status.status == "ok" && redis_status.status == "ok" {
        "ok"
    } else {
        "degraded"
    };

    Json(ReadinessResponse {
        status: overall_status.to_string(),
        database: db_status,
        redis: redis_status,
    })
}

async fn check_database(state: &AppState) -> ServiceStatus {
    match state.db_pool.get() {
        Ok(mut conn) => {
            use diesel::prelude::*;
            match diesel::sql_query("SELECT 1").execute(&mut conn) {
                Ok(_) => ServiceStatus {
                    status: "ok".to_string(),
                    message: None,
                },
                Err(e) => ServiceStatus {
                    status: "error".to_string(),
                    message: Some(e.to_string()),
                },
            }
        }
        Err(e) => ServiceStatus {
            status: "error".to_string(),
            message: Some(e.to_string()),
        },
    }
}

async fn check_redis(state: &AppState) -> ServiceStatus {
    match state.redis_pool.get().await {
        Ok(mut conn) => {
            let result: Result<String, _> = redis::cmd("PING").query_async(&mut conn).await;
            match result {
                Ok(_) => ServiceStatus {
                    status: "ok".to_string(),
                    message: None,
                },
                Err(e) => ServiceStatus {
                    status: "error".to_string(),
                    message: Some(e.to_string()),
                },
            }
        }
        Err(e) => ServiceStatus {
            status: "error".to_string(),
            message: Some(e.to_string()),
        },
    }
}
