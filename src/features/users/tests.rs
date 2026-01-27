use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::users::dto::CreateUserRequest;
    use validator::Validate;

    #[test]
    fn test_create_user_request_validation() {
        let valid_request = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_email = CreateUserRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };
        assert!(invalid_email.validate().is_err());

        let short_password = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
            name: "Test User".to_string(),
        };
        assert!(short_password.validate().is_err());

        let empty_name = CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            name: "".to_string(),
        };
        assert!(empty_name.validate().is_err());
    }

    #[test]
    fn test_update_user_request_validation() {
        let valid_request = UpdateUserRequest {
            email: Some("new@example.com".to_string()),
            name: Some("New Name".to_string()),
        };
        assert!(valid_request.validate().is_ok());

        let invalid_email = UpdateUserRequest {
            email: Some("invalid".to_string()),
            name: None,
        };
        assert!(invalid_email.validate().is_err());
    }

    #[test]
    fn test_user_response_from_user() {
        use chrono::Utc;
        use uuid::Uuid;

        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            name: "Test".to_string(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let response: UserResponse = user.clone().into();
        assert_eq!(response.id, user.id);
        assert_eq!(response.email, user.email);
        assert_eq!(response.name, user.name);
    }
}
