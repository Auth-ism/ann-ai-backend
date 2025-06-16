
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use sqlx::FromRow; 


#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub phone_number: Option<String>,
    pub token_balance: Option<f64>,
    pub user_role: UserRole,
    pub subscription_expries: Option<NaiveDateTime>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub enum UserRole {
    User,
    Admin,
    Guest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
}

// User login DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub username_or_email: String,
    pub password: String,
}

// DTO for user response (excludes sensitive data)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub token_balance: Option<BigDecimal>,
    pub user_role: UserRole,
    pub subscription_expires: Option<DateTime<Utc>>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap(),
            username: user.username,
            full_name: user.full_name,
            email: user.email,
            phone_number: user.phone_number,
            token_balance: user.token_balance,
            user_role: user.user_role,
            subscription_expires: user.subscription_expires,
            email_verified: user.email_verified,
            phone_verified: user.phone_verified,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}
