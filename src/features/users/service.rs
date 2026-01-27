use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use chrono::Utc;
use password_hash::rand_core::OsRng;
use uuid::Uuid;

use super::{CreateUserRequest, NewUser, UpdateUser, UpdateUserRequest, User, UserRepository};
use crate::config::DbPool;
use crate::core::{ApiError, Paginated, PaginationParams};

pub struct UserService;

impl UserService {
    pub fn find_by_id(pool: &DbPool, id: Uuid) -> Result<User, ApiError> {
        let mut conn = pool.get()?;
        UserRepository::find_by_id(&mut conn, id)
    }

    pub fn find_by_email(pool: &DbPool, email: &str) -> Result<Option<User>, ApiError> {
        let mut conn = pool.get()?;
        UserRepository::find_by_email(&mut conn, email)
    }

    pub fn list(pool: &DbPool, params: &PaginationParams) -> Result<Paginated<User>, ApiError> {
        let mut conn = pool.get()?;
        let users = UserRepository::find_all(&mut conn, params.limit(), params.offset())?;
        let total = UserRepository::count(&mut conn)?;

        Ok(Paginated::new(users, total, params))
    }

    pub fn create(pool: &DbPool, request: CreateUserRequest) -> Result<User, ApiError> {
        let mut conn = pool.get()?;

        // Check if email already exists
        if UserRepository::exists_by_email(&mut conn, &request.email)? {
            return Err(ApiError::Conflict("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = hash_password(&request.password)?;

        let new_user = NewUser {
            id: Uuid::new_v4(),
            email: request.email,
            password_hash,
            name: request.name,
        };

        UserRepository::create(&mut conn, new_user)
    }

    pub fn update(pool: &DbPool, id: Uuid, request: UpdateUserRequest) -> Result<User, ApiError> {
        let mut conn = pool.get()?;

        // Check if user exists
        UserRepository::find_by_id(&mut conn, id)?;

        // Check if new email is already taken by another user
        if let Some(ref email) = request.email
            && let Some(existing) = UserRepository::find_by_email(&mut conn, email)?
            && existing.id != id
        {
            return Err(ApiError::Conflict("Email already registered".to_string()));
        }

        let update = UpdateUser {
            email: request.email,
            name: request.name,
            is_active: None,
            updated_at: Utc::now(),
        };

        UserRepository::update(&mut conn, id, update)
    }

    pub fn delete(pool: &DbPool, id: Uuid) -> Result<(), ApiError> {
        let mut conn = pool.get()?;

        // Check if user exists
        UserRepository::find_by_id(&mut conn, id)?;

        UserRepository::delete(&mut conn, id)?;
        Ok(())
    }
}

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| ApiError::InternalServerError)
}
