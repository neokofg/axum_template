// use axum::{
//     routing::{delete, get, post},
//     Router,
// };
//
// use super::handlers;
// use crate::AppState;
//
// pub fn router() -> Router<AppState> {
//     Router::new()
//         .route("/", get(handlers::list))
//         .route("/", post(handlers::create))
//         .route("/{id}", get(handlers::get))
//         .route("/{id}", delete(handlers::delete))
// }

use axum::Router;
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}
