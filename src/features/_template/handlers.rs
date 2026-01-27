// use axum::{
//     extract::{Path, Query, State},
// };
//
// use super::{CreateRequest, Response, YourService};
// use crate::core::extractors::ValidatedJson;
// use crate::core::{ApiError, ApiResponse, Created, NoContent, PaginationParams};
// use crate::AppState;
//
// pub async fn list(
//     State(state): State<AppState>,
//     Query(params): Query<PaginationParams>,
// ) -> Result<ApiResponse<Vec<Response>>, ApiError> {
//     let paginated = YourService::list(&state.db_pool, &params)?;
//     let items: Vec<Response> = paginated.items.into_iter().map(Into::into).collect();
//     let meta = paginated.meta();
//
//     Ok(ApiResponse::with_meta(items, meta))
// }
//
// pub async fn get(
//     State(state): State<AppState>,
//     Path(id): Path<String>,
// ) -> Result<ApiResponse<Response>, ApiError> {
//     let model = YourService::find_by_id(&state.db_pool, &id)?;
//     Ok(ApiResponse::success(model.into()))
// }
//
// pub async fn create(
//     State(state): State<AppState>,
//     ValidatedJson(request): ValidatedJson<CreateRequest>,
// ) -> Result<Created<Response>, ApiError> {
//     let model = YourService::create(&state.db_pool, request)?;
//     Ok(Created(model.into()))
// }
//
// pub async fn delete(
//     State(state): State<AppState>,
//     Path(id): Path<String>,
// ) -> Result<NoContent, ApiError> {
//     YourService::delete(&state.db_pool, &id)?;
//     Ok(NoContent)
// }
