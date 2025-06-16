
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use sqlx::FromRow; 

// src/models/user.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use sqlx::FromRow; // Veritabanından satırları struct'a eşlemek için

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
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
