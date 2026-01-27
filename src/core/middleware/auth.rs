use axum::{
    body::Body,
    extract::State,
    http::{Request, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::AppState;
use crate::core::ApiError;
use crate::core::extractors::CurrentUser;
use crate::features::auth::service::Claims;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let token = extract_token(&request)?;

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(state.settings.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| ApiError::JwtError(e.to_string()))?
    .claims;

    let current_user = CurrentUser {
        id: claims.sub,
        email: claims.email,
    };

    request.extensions_mut().insert(current_user);

    Ok(next.run(request).await)
}

fn extract_token<B>(request: &Request<B>) -> Result<String, ApiError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::Unauthorized);
    }

    Ok(auth_header[7..].to_string())
}
