// use diesel::prelude::*;
//
// use super::{NewYourModel, UpdateYourModel, YourModel};
// use crate::config::DbConnection;
// use crate::core::ApiError;
// use crate::schema::your_table;
//
// pub struct YourRepository;
//
// impl YourRepository {
//     pub fn find_by_id(conn: &mut DbConnection, id: &str) -> Result<YourModel, ApiError> {
//         your_table::table
//             .filter(your_table::id.eq(id))
//             .first(conn)
//             .map_err(ApiError::from)
//     }
//
//     pub fn find_all(
//         conn: &mut DbConnection,
//         limit: i64,
//         offset: i64,
//     ) -> Result<Vec<YourModel>, ApiError> {
//         your_table::table
//             .order(your_table::created_at.desc())
//             .limit(limit)
//             .offset(offset)
//             .load(conn)
//             .map_err(ApiError::from)
//     }
//
//     pub fn count(conn: &mut DbConnection) -> Result<i64, ApiError> {
//         your_table::table
//             .count()
//             .get_result(conn)
//             .map_err(ApiError::from)
//     }
//
//     pub fn create(conn: &mut DbConnection, new_model: NewYourModel) -> Result<YourModel, ApiError> {
//         diesel::insert_into(your_table::table)
//             .values(&new_model)
//             .returning(YourModel::as_returning())
//             .get_result(conn)
//             .map_err(ApiError::from)
//     }
//
//     pub fn update(
//         conn: &mut DbConnection,
//         id: &str,
//         update_model: UpdateYourModel,
//     ) -> Result<YourModel, ApiError> {
//         diesel::update(your_table::table.filter(your_table::id.eq(id)))
//             .set(&update_model)
//             .returning(YourModel::as_returning())
//             .get_result(conn)
//             .map_err(ApiError::from)
//     }
//
//     pub fn delete(conn: &mut DbConnection, id: &str) -> Result<usize, ApiError> {
//         diesel::delete(your_table::table.filter(your_table::id.eq(id)))
//             .execute(conn)
//             .map_err(ApiError::from)
//     }
// }
