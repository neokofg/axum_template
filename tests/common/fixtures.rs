use fake::{faker::internet::en::SafeEmail, faker::name::en::Name, Fake};
use uuid::Uuid;

pub struct UserFixture {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
}

impl UserFixture {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            email: SafeEmail().fake(),
            password: "password123".to_string(),
            name: Name().fake(),
        }
    }

    pub fn with_email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

impl Default for UserFixture {
    fn default() -> Self {
        Self::new()
    }
}
