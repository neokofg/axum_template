use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::auth::dto::{LoginRequest, RegisterRequest};
    use validator::Validate;

    #[test]
    fn test_register_request_validation() {
        let valid_request = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_email = RegisterRequest {
            email: "invalid".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };
        assert!(invalid_email.validate().is_err());
    }

    #[test]
    fn test_login_request_validation() {
        let valid_request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_email = LoginRequest {
            email: "invalid".to_string(),
            password: "password123".to_string(),
        };
        assert!(invalid_email.validate().is_err());

        let empty_password = LoginRequest {
            email: "test@example.com".to_string(),
            password: "".to_string(),
        };
        assert!(empty_password.validate().is_err());
    }

    #[test]
    fn test_token_response_creation() {
        let response =
            TokenResponse::new("access_token".to_string(), "refresh_token".to_string(), 900);

        assert_eq!(response.access_token, "access_token");
        assert_eq!(response.refresh_token, "refresh_token");
        assert_eq!(response.token_type, "Bearer");
        assert_eq!(response.expires_in, 900);
    }
}
