use axum::{middleware, Router};
use axum_template::{
    config::{create_pool, Settings},
    core::middleware::{auth_middleware, request_id_middleware},
    features,
    infrastructure::cache::create_redis_pool,
    AppState,
};
use tower_http::trace::TraceLayer;

pub struct TestApp {
    pub state: AppState,
}

impl TestApp {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();

        let settings = Settings::new().expect("Failed to load settings");
        let db_pool = create_pool(&settings.database);
        let redis_pool = create_redis_pool(&settings.redis)
            .await
            .expect("Failed to create Redis pool");

        let state = AppState::new(settings, db_pool, redis_pool);

        Self { state }
    }

    pub fn router(&self) -> Router {
        let state = self.state.clone();

        let protected_users = features::users::protected_router()
            .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

        let api_v1 = Router::new()
            .nest("/auth", features::auth::router())
            .nest("/users", features::users::public_router())
            .nest("/users", protected_users);

        Router::new()
            .merge(features::health::router())
            .nest("/api/v1", api_v1)
            .layer(TraceLayer::new_for_http())
            .layer(middleware::from_fn(request_id_middleware))
            .with_state(state)
    }
}
