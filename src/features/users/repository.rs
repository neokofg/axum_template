use diesel::prelude::*;
use uuid::Uuid;

use super::{NewUser, UpdateUser, User};
use crate::config::DbConnection;
use crate::core::ApiError;
use crate::schema::users;

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_id(conn: &mut DbConnection, id: Uuid) -> Result<User, ApiError> {
        users::table
            .filter(users::id.eq(id))
            .first(conn)
            .map_err(ApiError::from)
    }

    pub fn find_by_email(conn: &mut DbConnection, email: &str) -> Result<Option<User>, ApiError> {
        users::table
            .filter(users::email.eq(email))
            .first(conn)
            .optional()
            .map_err(ApiError::from)
    }

    pub fn find_all(
        conn: &mut DbConnection,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, ApiError> {
        users::table
            .order(users::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load(conn)
            .map_err(ApiError::from)
    }

    pub fn count(conn: &mut DbConnection) -> Result<i64, ApiError> {
        users::table
            .count()
            .get_result(conn)
            .map_err(ApiError::from)
    }

    pub fn create(conn: &mut DbConnection, new_user: NewUser) -> Result<User, ApiError> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .map_err(ApiError::from)
    }

    pub fn update(
        conn: &mut DbConnection,
        id: Uuid,
        update_user: UpdateUser,
    ) -> Result<User, ApiError> {
        diesel::update(users::table.filter(users::id.eq(id)))
            .set(&update_user)
            .returning(User::as_returning())
            .get_result(conn)
            .map_err(ApiError::from)
    }

    pub fn delete(conn: &mut DbConnection, id: Uuid) -> Result<usize, ApiError> {
        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn)
            .map_err(ApiError::from)
    }

    pub fn exists_by_email(conn: &mut DbConnection, email: &str) -> Result<bool, ApiError> {
        use diesel::dsl::exists;
        diesel::select(exists(users::table.filter(users::email.eq(email))))
            .get_result(conn)
            .map_err(ApiError::from)
    }
}
