use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handlers;
use crate::AppState;

pub fn public_router() -> Router<AppState> {
    Router::new().route("/", post(handlers::create_user))
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_users))
        .route("/me", get(handlers::get_current_user))
        .route("/me", put(handlers::update_current_user))
        .route("/{id}", get(handlers::get_user))
        .route("/{id}", put(handlers::update_user))
        .route("/{id}", delete(handlers::delete_user))
}
