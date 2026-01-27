pub mod config;
pub mod core;
pub mod features;
pub mod infrastructure;
pub mod schema;

use std::sync::Arc;

use config::{DbPool, Settings};
use infrastructure::cache::RedisPool;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub db_pool: DbPool,
    pub redis_pool: RedisPool,
}

impl AppState {
    pub fn new(settings: Settings, db_pool: DbPool, redis_pool: RedisPool) -> Self {
        Self {
            settings: Arc::new(settings),
            db_pool,
            redis_pool,
        }
    }
}

pub use core::ApiError;
