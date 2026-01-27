use axum::{extract::FromRequestParts, http::request::Parts};

use crate::core::ApiError;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: String,
    pub email: String,
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    // Cannot use `async fn` due to trait bounds requiring explicit future type
    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            parts
                .extensions
                .get::<CurrentUser>()
                .cloned()
                .ok_or(ApiError::Unauthorized)
        }
    }
}
