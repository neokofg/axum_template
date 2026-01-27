use axum::extract::{Path, Query, State};

use super::{CreateUserRequest, UpdateUserRequest, UserResponse, UserService};
use crate::AppState;
use crate::core::extractors::{CurrentUser, ValidatedJson};
use crate::core::{ApiError, ApiResponse, Created, NoContent, PaginationParams};

pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<ApiResponse<Vec<UserResponse>>, ApiError> {
    let paginated = UserService::list(&state.db_pool, &params)?;
    let meta = paginated.meta();
    let users: Vec<UserResponse> = paginated.items.into_iter().map(Into::into).collect();

    Ok(ApiResponse::with_meta(users, meta))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = UserService::find_by_id(&state.db_pool, &id)?;
    Ok(ApiResponse::success(user.into()))
}

pub async fn get_current_user(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = UserService::find_by_id(&state.db_pool, &current_user.id)?;
    Ok(ApiResponse::success(user.into()))
}

pub async fn create_user(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> Result<Created<UserResponse>, ApiError> {
    let user = UserService::create(&state.db_pool, request)?;
    Ok(Created(user.into()))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    ValidatedJson(request): ValidatedJson<UpdateUserRequest>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = UserService::update(&state.db_pool, &id, request)?;
    Ok(ApiResponse::success(user.into()))
}

pub async fn update_current_user(
    State(state): State<AppState>,
    current_user: CurrentUser,
    ValidatedJson(request): ValidatedJson<UpdateUserRequest>,
) -> Result<ApiResponse<UserResponse>, ApiError> {
    let user = UserService::update(&state.db_pool, &current_user.id, request)?;
    Ok(ApiResponse::success(user.into()))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<NoContent, ApiError> {
    UserService::delete(&state.db_pool, &id)?;
    Ok(NoContent)
}
