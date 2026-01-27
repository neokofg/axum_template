// use uuid::Uuid;
//
// use super::{CreateRequest, YourModel, YourRepository};
// use crate::config::DbPool;
// use crate::core::{ApiError, Paginated, PaginationParams};
//
// pub struct YourService;
//
// impl YourService {
//     pub fn find_by_id(pool: &DbPool, id: Uuid) -> Result<YourModel, ApiError> {
//         let mut conn = pool.get()?;
//         YourRepository::find_by_id(&mut conn, id)
//     }
//
//     pub fn list(pool: &DbPool, params: &PaginationParams) -> Result<Paginated<YourModel>, ApiError> {
//         let mut conn = pool.get()?;
//         let items = YourRepository::find_all(&mut conn, params.limit(), params.offset())?;
//         let total = YourRepository::count(&mut conn)?;
//
//         Ok(Paginated::new(items, total, params))
//     }
//
//     pub fn create(pool: &DbPool, request: CreateRequest) -> Result<YourModel, ApiError> {
//         let mut conn = pool.get()?;
//         // ... implement creation logic
//         todo!()
//     }
//
//     pub fn delete(pool: &DbPool, id: Uuid) -> Result<(), ApiError> {
//         let mut conn = pool.get()?;
//         YourRepository::find_by_id(&mut conn, id)?;
//         YourRepository::delete(&mut conn, id)?;
//         Ok(())
//     }
// }
