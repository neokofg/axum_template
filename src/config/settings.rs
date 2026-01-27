use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub jwt: JwtSettings,
    pub rate_limit: RateLimitSettings,
    pub logging: LoggingSettings,
    pub smtp: SmtpSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub name: String,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisSettings {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: String,
    pub access_token_expires_in: String,
    pub refresh_token_expires_in: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitSettings {
    pub requests_per_second: u32,
    pub burst_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SmtpSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub from_name: String,
    pub tls: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            // Start with default settings
            .add_source(File::with_name("config/default"))
            // Layer on the environment-specific settings
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add environment variables (with prefix APP_)
            // e.g., `APP_DATABASE__URL=xxx` sets `database.url`
            .add_source(Environment::default().separator("__").try_parsing(true))
            // Override specific values from direct env vars
            .set_override_option("database.url", env::var("DATABASE_URL").ok())?
            .set_override_option("redis.url", env::var("REDIS_URL").ok())?
            .set_override_option("jwt.secret", env::var("JWT_SECRET").ok())?
            .set_override_option("app.host", env::var("APP_HOST").ok())?
            .set_override_option(
                "app.port",
                env::var("APP_PORT")
                    .ok()
                    .and_then(|p| p.parse::<u16>().ok()),
            )?
            .set_override_option("smtp.host", env::var("SMTP_HOST").ok())?
            .set_override_option(
                "smtp.port",
                env::var("SMTP_PORT")
                    .ok()
                    .and_then(|p| p.parse::<u16>().ok()),
            )?
            .set_override_option("smtp.username", env::var("SMTP_USERNAME").ok())?
            .set_override_option("smtp.password", env::var("SMTP_PASSWORD").ok())?
            .set_override_option("smtp.from_email", env::var("SMTP_FROM_EMAIL").ok())?
            .set_override_option("smtp.from_name", env::var("SMTP_FROM_NAME").ok())?
            .build()?;

        config.try_deserialize()
    }

    pub fn is_production(&self) -> bool {
        env::var("APP_ENV")
            .map(|e| e == "production")
            .unwrap_or(false)
    }

    pub fn is_development(&self) -> bool {
        env::var("APP_ENV")
            .map(|e| e == "development")
            .unwrap_or(true)
    }
}

impl JwtSettings {
    pub fn access_token_duration(&self) -> chrono::Duration {
        parse_duration(&self.access_token_expires_in).unwrap_or(chrono::Duration::minutes(15))
    }

    pub fn refresh_token_duration(&self) -> chrono::Duration {
        parse_duration(&self.refresh_token_expires_in).unwrap_or(chrono::Duration::days(7))
    }
}

fn parse_duration(s: &str) -> Option<chrono::Duration> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (num, unit) = s.split_at(s.len() - 1);
    let num: i64 = num.parse().ok()?;

    match unit {
        "s" => Some(chrono::Duration::seconds(num)),
        "m" => Some(chrono::Duration::minutes(num)),
        "h" => Some(chrono::Duration::hours(num)),
        "d" => Some(chrono::Duration::days(num)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("15m"), Some(chrono::Duration::minutes(15)));
        assert_eq!(parse_duration("7d"), Some(chrono::Duration::days(7)));
        assert_eq!(parse_duration("1h"), Some(chrono::Duration::hours(1)));
        assert_eq!(parse_duration("30s"), Some(chrono::Duration::seconds(30)));
        assert_eq!(parse_duration("invalid"), None);
    }
}
