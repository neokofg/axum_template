use redis::AsyncCommands;
use serde::Serialize;
use tracing::info;

use crate::infrastructure::cache::RedisPool;

#[derive(Clone)]
pub struct QueueClient {
    pool: RedisPool,
    default_queue: String,
}

impl QueueClient {
    pub fn new(pool: RedisPool) -> Self {
        Self {
            pool,
            default_queue: "default".to_string(),
        }
    }

    pub fn with_queue(mut self, queue: &str) -> Self {
        self.default_queue = queue.to_string();
        self
    }

    pub async fn enqueue<T: Serialize>(
        &self,
        job_type: &str,
        args: T,
    ) -> Result<String, redis::RedisError> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let job = serde_json::json!({
            "id": job_id,
            "type": job_type,
            "args": args,
            "queue": self.default_queue,
            "created_at": chrono::Utc::now().to_rfc3339(),
        });

        let mut conn = self.pool.get().await?;
        let key = format!("queue:{}", self.default_queue);
        conn.rpush::<_, _, ()>(&key, serde_json::to_string(&job).unwrap())
            .await?;

        info!(job_id = %job_id, job_type = %job_type, queue = %self.default_queue, "Job enqueued");
        Ok(job_id)
    }

    pub async fn enqueue_in<T: Serialize>(
        &self,
        job_type: &str,
        args: T,
        delay_seconds: i64,
    ) -> Result<String, redis::RedisError> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let execute_at = chrono::Utc::now() + chrono::Duration::seconds(delay_seconds);
        let job = serde_json::json!({
            "id": job_id,
            "type": job_type,
            "args": args,
            "queue": self.default_queue,
            "created_at": chrono::Utc::now().to_rfc3339(),
            "execute_at": execute_at.to_rfc3339(),
        });

        let mut conn = self.pool.get().await?;
        let key = "queue:scheduled";
        conn.zadd::<_, _, _, ()>(
            key,
            serde_json::to_string(&job).unwrap(),
            execute_at.timestamp() as f64,
        )
        .await?;

        info!(
            job_id = %job_id,
            job_type = %job_type,
            delay_seconds = delay_seconds,
            "Job scheduled"
        );
        Ok(job_id)
    }

    pub async fn dequeue(&self) -> Result<Option<serde_json::Value>, redis::RedisError> {
        let mut conn = self.pool.get().await?;
        let key = format!("queue:{}", self.default_queue);
        let result: Option<String> = conn.lpop(&key, None).await?;

        match result {
            Some(job_str) => {
                let job: serde_json::Value = serde_json::from_str(&job_str).unwrap();
                Ok(Some(job))
            }
            None => Ok(None),
        }
    }
}
