use serde::{Deserialize, Serialize};
use validator::Validate;

/// Kullanıcı güncelleme isteği DTO'su
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserUpdate {
    pub id: i32,
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(length(min = 3, max = 80))]
    pub full_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(equal = 10, message = "Phone number must be 10 digits"))]
    pub phone_number: Option<String>,
}

/// Son kaydolan kullanıcılar için sorgu DTO'su
#[derive(Debug, Deserialize)]
pub struct RecentUsersQuery {
    #[serde(default = "default_days")]
    pub days: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_days() -> i64 { 7 }
fn default_limit() -> i64 { 10 }

/// Şifre değiştirme isteği DTO'su
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordChangeRequest {
    #[validate(length(min = 8, message = "Current password must be at least 8 characters"))]
    pub current_password: String,
    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

/// Sayfalama için sorgu DTO'su
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
}

fn default_page() -> usize { 0 }
fn default_page_size() -> usize { 10 }

/// Kullanıcı yanıt DTO'su (kullanıcı bilgilerini dışarıya açmak için)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub role: String,
}

impl From<crate::models::user::User> for UserResponse {
    fn from(user: crate::models::user::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            full_name: user.full_name,
            email: user.email,
            phone_number: user.phone_number,
            role: user.user_role.to_string(),
        }
    }
}