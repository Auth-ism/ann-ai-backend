use actix_web::web;
use chrono::DateTime;
use log::{debug, error};
use validator::Validate;
use uuid::Uuid;
use crate::{
    app_state::{self, AppState}, error::AppError, 
    models::{dto::{auth::{AuthResponse, Claims, LoginRequest, RegisterRequest }, user::UserUpdate},
    user::{User, UserRole, UserSchema}},
    repositories::{self, auth::find_by_username_or_email, user::{find_user_by_email, find_user_by_id, update_user}}, 
    result::AppResult, 
    utils::{jwt, password, sha256::sha256_hash, uudi_convert_32byte}
    
};
use deadpool_redis::redis::AsyncCommands; // Redis komutları için


pub async fn find_user_id(
    app_state: &web::Data<AppState>,
    path: web::Path<i32>,
) -> AppResult<User> {

    let user_id = path.into_inner();
    let user = find_user_by_id(&app_state.db, user_id)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;
    Ok(user)
}

pub async fn find_user_mail(
    app_state: &web::Data<AppState>,
    path: web::Path<String>,
) -> AppResult<User> {

    let user_email = path.into_inner();
    let user = find_user_by_email(&app_state.db, &user_email)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("User with email {} not found", user_email)))?;
    Ok(user)
}


pub async fn update_user_service(
    app_state: &web::Data<AppState>,
    req: UserUpdate
) -> AppResult<User> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let updated_user = update_user(&app_state.db, &req)
        .await?;

    Ok(updated_user)
}


/// Kullanıcı şifresini değiştirme
pub async fn change_password(
    app_state: &web::Data<AppState>,
    user_id: i32,
    current_password: &str,
    new_password: &str
) -> AppResult<()> {
    // Kullanıcıyı bul
    let user = find_user_by_id(&app_state.db, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;
    
    // Mevcut şifreyi kontrol et
    if !password::verify_password(current_password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Current password is incorrect".to_string()));
    }
    
    // Yeni şifreyi hashle
    let new_password_hash = password::hash_password(new_password)?;
    
    // Şifreyi güncelle
    repositories::user::update_password(&app_state.db, user_id, &new_password_hash).await?;
    
    Ok(())
}

/// Sayfalanmış kullanıcı listesi (admin)
pub async fn list_users(
    app_state: &web::Data<AppState>,
    page: usize,
    page_size: usize
) -> AppResult<(Vec<User>, i64)> {
    let limit = page_size as i64;
    let offset = (page * page_size) as i64;
    
    // Toplam kullanıcı sayısını al
    let total = repositories::user::count_users(&app_state.db).await?;
    
    // Sayfalanmış kullanıcıları al
    let users = repositories::user::get_users(&app_state.db, limit, offset).await?;
    
    Ok((users, total))
}

/// Kullanıcı arama
pub async fn search_users_service(
    app_state: &web::Data<AppState>,
    query: &str,
    page: usize,
    page_size: usize
) -> AppResult<Vec<User>> {
    let limit = page_size as i64;
    let offset = (page * page_size) as i64;
    
    repositories::user::search_users(&app_state.db, query, limit, offset).await
}

/// Kullanıcı rolü güncelleme (admin için)
pub async fn update_user_role_service(
    app_state: &web::Data<AppState>,
    user_id: i32,
    role: UserRole
) -> AppResult<()> {
    // Kullanıcının var olduğunu kontrol et
    let _ = find_user_by_id(&app_state.db, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;
    
    // Rolü güncelle
    repositories::user::update_user_role(&app_state.db, user_id, &role.to_string()).await?;
    
    Ok(())
}

/// Kullanıcı hesabını devre dışı bırakma
pub async fn deactivate_user_service(
    app_state: &web::Data<AppState>,
    user_id: i32
) -> AppResult<()> {
    // Kullanıcının var olduğunu kontrol et
    let _ = find_user_by_id(&app_state.db, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;
    
    repositories::user::deactivate_user(&app_state.db, user_id).await?;
    
    Ok(())
}

/// Kullanıcı hesabını yeniden aktifleştirme
pub async fn reactivate_user_service(
    app_state: &web::Data<AppState>,
    user_id: i32
) -> AppResult<()> {
    // Kullanıcının var olduğunu kontrol et
    let _ = find_user_by_id(&app_state.db, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;
    
    repositories::user::reactivate_user(&app_state.db, user_id).await?;
    
    Ok(())
}

/// E-posta doğrulama işaretleme
pub async fn mark_email_verified(
    app_state: &web::Data<AppState>,
    user_id: i32
) -> AppResult<()> {
    repositories::user::set_email_verified(&app_state.db, user_id, true).await?;
    Ok(())
}

/// Son kaydolan kullanıcıları alma (dashboard için)
pub async fn get_recent_users_service(
    app_state: &web::Data<AppState>,
    days: i64,
    limit: i64
) -> AppResult<Vec<User>> {
    repositories::user::get_recent_users(&app_state.db, days, limit).await
}

/// Kullanıcı giriş zamanını güncelleme (login sırasında çağrılır)
pub async fn update_login_timestamp(
    app_state: &web::Data<AppState>,
    user_id: i32
) -> AppResult<()> {
    repositories::user::update_last_login(&app_state.db, user_id).await?;
    Ok(())
}

