use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::core::ApiError;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ApiError;

    // Cannot use `async fn` due to trait bounds requiring explicit future type
    #[allow(clippy::manual_async_fn)]
    fn from_request(
        req: Request,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection| ApiError::BadRequest(rejection.body_text()))?;

            value.validate()?;

            Ok(ValidatedJson(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Validate)]
    struct TestPayload {
        #[validate(length(min = 1, max = 100))]
        name: String,
        #[validate(email)]
        email: String,
    }

    #[test]
    fn test_validate_payload() {
        let valid_payload = TestPayload {
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
        };
        assert!(valid_payload.validate().is_ok());

        let invalid_payload = TestPayload {
            name: "".to_string(),
            email: "invalid-email".to_string(),
        };
        assert!(invalid_payload.validate().is_err());
    }
}
