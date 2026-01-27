use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::time::Duration;

use super::DatabaseSettings;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool(settings: &DatabaseSettings) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(&settings.url);

    Pool::builder()
        .max_size(settings.pool_size)
        .connection_timeout(Duration::from_secs(settings.connection_timeout_secs))
        .build(manager)
        .expect("Failed to create database pool")
}

#[cfg(test)]
pub fn create_test_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(5)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .expect("Failed to create test database pool")
}
