use std::fmt;
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Admin,
    Guest,
}

impl From<String> for UserRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "admin" => UserRole::Admin,
            "user" => UserRole::User,
            _ => UserRole::User, 
        }
    }
}
// Add Display implementation for UserRole
impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::Guest => write!(f, "guest"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSchema {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub phone_number: Option<String>,
    pub token_balance: Option<BigDecimal>,
    pub user_role: UserRole,
    pub subscription_expries: Option<DateTime<Utc>>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// User model with f64 token balance for application use
#[derive(Debug, Serialize, Deserialize)]
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
    pub subscription_expries: Option<DateTime<Utc>>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<UserSchema> for User {
    type Error = AppError;
    
    fn try_from(value: UserSchema) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            username: value.username,
            full_name: value.full_name,
            email: value.email,
            password_hash: value.password_hash,
            phone_number: value.phone_number,
            token_balance: value.token_balance.and_then(|bd| bd.to_f64()),
            user_role: value.user_role,
            subscription_expries: value.subscription_expries,
            email_verified: value.email_verified,
            phone_verified: value.phone_verified,
            last_login: value.last_login,
            is_active: value.is_active,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}