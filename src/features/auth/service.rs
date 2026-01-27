use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{AuthResponse, LoginRequest, RegisterRequest, TokenResponse, UserInfo};
use crate::config::{DbPool, JwtSettings};
use crate::core::ApiError;
use crate::features::users::{CreateUserRequest, User, UserService};
use crate::infrastructure::cache::RedisPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct AuthService;

impl AuthService {
    pub async fn register(
        db_pool: &DbPool,
        redis_pool: &RedisPool,
        jwt_settings: &JwtSettings,
        request: RegisterRequest,
    ) -> Result<AuthResponse, ApiError> {
        let create_request = CreateUserRequest {
            email: request.email,
            password: request.password,
            name: request.name,
        };

        let user = UserService::create(db_pool, create_request)?;

        let tokens = Self::generate_tokens(&user, jwt_settings, redis_pool).await?;

        Ok(AuthResponse {
            user: UserInfo {
                id: user.id.to_string(),
                email: user.email,
                name: user.name,
            },
            tokens,
        })
    }

    pub async fn login(
        db_pool: &DbPool,
        redis_pool: &RedisPool,
        jwt_settings: &JwtSettings,
        request: LoginRequest,
    ) -> Result<AuthResponse, ApiError> {
        let user =
            UserService::find_by_email(db_pool, &request.email)?.ok_or(ApiError::Unauthorized)?;

        if !user.is_active {
            return Err(ApiError::Forbidden);
        }

        Self::verify_password(&request.password, &user.password_hash)?;

        let tokens = Self::generate_tokens(&user, jwt_settings, redis_pool).await?;

        Ok(AuthResponse {
            user: UserInfo {
                id: user.id.to_string(),
                email: user.email,
                name: user.name,
            },
            tokens,
        })
    }

    pub async fn refresh(
        db_pool: &DbPool,
        redis_pool: &RedisPool,
        jwt_settings: &JwtSettings,
        refresh_token: &str,
    ) -> Result<TokenResponse, ApiError> {
        // Verify refresh token exists in Redis
        let token_key = format!("refresh_token:{}", refresh_token);
        let mut conn = redis_pool.get().await?;

        let user_id: Option<String> = conn.get(&token_key).await?;
        let user_id = user_id.ok_or(ApiError::Unauthorized)?;

        let user_uuid = Uuid::parse_str(&user_id).map_err(|_| ApiError::Unauthorized)?;

        let user = UserService::find_by_id(db_pool, user_uuid)?;

        // Delete old refresh token
        conn.del::<_, ()>(&token_key).await?;

        Self::generate_tokens(&user, jwt_settings, redis_pool).await
    }

    pub async fn logout(redis_pool: &RedisPool, refresh_token: &str) -> Result<(), ApiError> {
        let token_key = format!("refresh_token:{}", refresh_token);
        let mut conn = redis_pool.get().await?;
        conn.del::<_, ()>(&token_key).await?;
        Ok(())
    }

    async fn generate_tokens(
        user: &User,
        jwt_settings: &JwtSettings,
        redis_pool: &RedisPool,
    ) -> Result<TokenResponse, ApiError> {
        let now = Utc::now();
        let access_token_duration = jwt_settings.access_token_duration();
        let refresh_token_duration = jwt_settings.refresh_token_duration();

        // Generate access token
        let access_claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: (now + access_token_duration).timestamp(),
            iat: now.timestamp(),
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(jwt_settings.secret.as_bytes()),
        )?;

        // Generate refresh token
        let refresh_token = Uuid::new_v4().to_string();

        // Store refresh token in Redis
        let token_key = format!("refresh_token:{}", refresh_token);
        let mut conn = redis_pool.get().await?;
        conn.set_ex::<_, _, ()>(
            &token_key,
            user.id.to_string(),
            refresh_token_duration.num_seconds() as u64,
        )
        .await?;

        Ok(TokenResponse::new(
            access_token,
            refresh_token,
            access_token_duration.num_seconds(),
        ))
    }

    fn verify_password(password: &str, hash: &str) -> Result<(), ApiError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| ApiError::InternalServerError)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| ApiError::Unauthorized)
    }
}
