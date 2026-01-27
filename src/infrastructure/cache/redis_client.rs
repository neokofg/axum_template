use redis::aio::ConnectionManager;
use std::sync::Arc;

use crate::config::RedisSettings;

pub type RedisPool = Arc<RedisPoolInner>;

pub struct RedisPoolInner {
    manager: ConnectionManager,
}

impl RedisPoolInner {
    pub async fn get(&self) -> Result<ConnectionManager, redis::RedisError> {
        Ok(self.manager.clone())
    }
}

pub async fn create_redis_pool(settings: &RedisSettings) -> Result<RedisPool, redis::RedisError> {
    let client = redis::Client::open(settings.url.as_str())?;
    let manager = ConnectionManager::new(client).await?;

    Ok(Arc::new(RedisPoolInner { manager }))
}

#[derive(Clone)]
pub struct RedisClient {
    pool: RedisPool,
}

impl RedisClient {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.pool.get().await?;
        redis::cmd("GET").arg(key).query_async(&mut conn).await
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.pool.get().await?;
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut conn)
            .await
    }

    pub async fn set_ex(
        &self,
        key: &str,
        value: &str,
        seconds: u64,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.pool.get().await?;
        redis::cmd("SETEX")
            .arg(key)
            .arg(seconds as i64)
            .arg(value)
            .query_async(&mut conn)
            .await
    }

    pub async fn del(&self, key: &str) -> Result<(), redis::RedisError> {
        let mut conn = self.pool.get().await?;
        redis::cmd("DEL").arg(key).query_async(&mut conn).await
    }

    pub async fn exists(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.pool.get().await?;
        redis::cmd("EXISTS").arg(key).query_async(&mut conn).await
    }
}
