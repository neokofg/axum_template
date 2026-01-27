use std::net::SocketAddr;

use axum::{Router, middleware};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum_template::{
    AppState,
    config::{Settings, create_pool},
    core::middleware::{auth_middleware, request_id_middleware},
    features,
    infrastructure::cache::create_redis_pool,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Load settings
    let settings = Settings::new()?;

    // Initialize tracing
    init_tracing(&settings);

    info!(
        "Starting {} v{}",
        settings.app.name,
        env!("CARGO_PKG_VERSION")
    );

    // Create database pool
    let db_pool = create_pool(&settings.database);
    info!("Database pool created");

    // Create Redis pool
    let redis_pool = create_redis_pool(&settings.redis).await?;
    info!("Redis pool created");

    // Create app state
    let state = AppState::new(settings.clone(), db_pool, redis_pool);

    // Build router
    let app = create_router(state.clone());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.app.port));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    // Protected routes (require authentication)
    let protected_users = features::users::protected_router().route_layer(
        middleware::from_fn_with_state(state.clone(), auth_middleware),
    );

    // API v1 routes
    let api_v1 = Router::new()
        .nest("/auth", features::auth::router())
        .nest("/users", features::users::public_router())
        .nest("/users", protected_users);

    // Main router
    Router::new()
        .merge(features::health::router())
        .nest("/api/v1", api_v1)
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            std::time::Duration::from_secs(30),
        ))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(request_id_middleware))
        .with_state(state)
}

fn init_tracing(settings: &Settings) {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
            "{}={},tower_http=debug,axum::rejection=trace",
            env!("CARGO_PKG_NAME").replace('-', "_"),
            settings.logging.level
        )
        .into()
    });

    if settings.logging.format == "json" {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    }
}
