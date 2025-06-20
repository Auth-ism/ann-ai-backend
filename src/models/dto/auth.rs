use futures::Stream;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
#[derive(Debug, Deserialize, Serialize, Validate)]

pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 3, max = 80))]
    pub full_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(equal = 10))]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_code: Option<String>, 
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct AuthResponse  {
    pub token: String,
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Claims {
    pub user_id: i32,
    pub exp: usize,
    pub iat: usize,
    pub role: String,
}

impl Claims {
    pub fn from_user(user: &crate::models::user::User) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;
        let exp = now + 3600; 
        Claims {
            user_id: user.id,
            exp,
            iat: now,
            role: user.user_role.to_string().clone(),
        }
    }
}
