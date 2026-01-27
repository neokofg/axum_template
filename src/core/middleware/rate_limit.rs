use axum::{
    body::Body,
    extract::ConnectInfo,
    http::Request,
    response::{IntoResponse, Response},
};
use redis::AsyncCommands;
use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

use crate::config::RateLimitSettings;
use crate::core::ApiError;
use crate::infrastructure::cache::RedisPool;

#[derive(Clone)]
pub struct RateLimitLayer {
    redis_pool: RedisPool,
    settings: RateLimitSettings,
}

impl RateLimitLayer {
    pub fn new(redis_pool: RedisPool, settings: RateLimitSettings) -> Self {
        Self {
            redis_pool,
            settings,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            redis_pool: self.redis_pool.clone(),
            settings: self.settings.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    redis_pool: RedisPool,
    settings: RateLimitSettings,
}

impl<S> Service<Request<Body>> for RateLimitService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let redis_pool = self.redis_pool.clone();
        let settings = self.settings.clone();

        Box::pin(async move {
            let client_ip = request
                .extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|ci| ci.0.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let key = format!("rate_limit:{}", client_ip);
            let window_secs = 1;
            let max_requests = settings.requests_per_second;

            let allowed = check_rate_limit(&redis_pool, &key, max_requests, window_secs).await;

            match allowed {
                Ok(true) => inner.call(request).await,
                Ok(false) => Ok(ApiError::RateLimitExceeded.into_response()),
                Err(_) => {
                    // On Redis error, allow the request (fail open)
                    inner.call(request).await
                }
            }
        })
    }
}

async fn check_rate_limit(
    redis_pool: &RedisPool,
    key: &str,
    max_requests: u32,
    window_secs: u64,
) -> Result<bool, redis::RedisError> {
    let mut conn = redis_pool.get().await?;

    let current: i64 = conn.incr(key, 1).await?;

    if current == 1 {
        conn.expire::<_, ()>(key, window_secs as i64).await?;
    }

    Ok(current <= max_requests as i64)
}
