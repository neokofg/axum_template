use axum::{Json, extract::State};

use super::{
    AuthResponse, AuthService, LoginRequest, RefreshTokenRequest, RegisterRequest, TokenResponse,
};
use crate::AppState;
use crate::core::extractors::ValidatedJson;
use crate::core::{ApiError, ApiResponse, Created, NoContent};

pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<RegisterRequest>,
) -> Result<Created<AuthResponse>, ApiError> {
    let response = AuthService::register(
        &state.db_pool,
        &state.redis_pool,
        &state.settings.jwt,
        request,
    )
    .await?;

    Ok(Created(response))
}

pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<LoginRequest>,
) -> Result<ApiResponse<AuthResponse>, ApiError> {
    let response = AuthService::login(
        &state.db_pool,
        &state.redis_pool,
        &state.settings.jwt,
        request,
    )
    .await?;

    Ok(ApiResponse::success(response))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<ApiResponse<TokenResponse>, ApiError> {
    let tokens = AuthService::refresh(
        &state.db_pool,
        &state.redis_pool,
        &state.settings.jwt,
        &request.refresh_token,
    )
    .await?;

    Ok(ApiResponse::success(tokens))
}

pub async fn logout(
    State(state): State<AppState>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<NoContent, ApiError> {
    AuthService::logout(&state.redis_pool, &request.refresh_token).await?;
    Ok(NoContent)
}
