use std::sync::Arc;
// src/services/auth.rs
use actix_web::web;
use chrono::Utc;
use log::{debug, error};
use validator::Validate;
use uuid::Uuid;
use crate::{
    app_state::AppState, error::AppError, models::{dto::auth::{AuthResponse, Claims, LoginRequest, RegisterRequest },
    user::{User, UserRole, UserSchema}}, repositories::{self, auth::find_by_username_or_email}, utils::{jwt, password, sha256::sha256_hash, uudi_convert_32byte}

};
use deadpool_redis::redis::AsyncCommands; // Redis komutları için

pub async fn register_user(
    app_state: &web::Data<AppState>,
    req: RegisterRequest,
) -> Result<User, AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let hashed_password = password::hash_password(&req.password)?;
    
    let role = if let Some(admin_code) = &req.admin_code {
        // Güvenli bir karşılaştırma için constant-time karşılaştırma kullan
        let code_bytes = admin_code.as_bytes();
        let stored_bytes = app_state.config.admin_registration_code.as_bytes();

        // Farklı uzunluklar varsa hemen ret döndür
        if code_bytes.len() != stored_bytes.len() {
            return Err(AppError::Unauthorized("Geçersiz admin kodu.".to_string()));
        }

        // XOR ile karşılaştırma - zamanı sabit tutmak için 
        // tüm karakterleri her durumda kontrol eder
        let mut result = 0u8;
        for (a, b) in code_bytes.iter().zip(stored_bytes.iter()) {
            result |= a ^ b; // XOR - sadece baytlar eşitse 0 olur
        }

        if result == 0 {
            UserRole::Admin
        } else {
            return Err(AppError::Unauthorized("Geçersiz admin kodu.".to_string()));
        }
    } else {
        UserRole::User
    };

    let user = UserSchema {
        id: uudi_convert_32byte::convert_i32(Uuid::new_v4()),
        full_name: req.full_name,
        username: req.username,
        email: req.email,
        password_hash: hashed_password.clone(),
        phone_number: req.phone_number,
        token_balance: None,
        user_role: role.clone(),  // Clone the role before first use
        subscription_expries: None,
        email_verified: None,
        phone_verified: None,
        last_login: None,
        is_active: Some(true),
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now())
    };
    
    let user = repositories::auth::create(
        &app_state.db,
        &user,
        &hashed_password,
        role
    )
    .await
    .map_err(|e| {
        // Try to match the error if it's an AppError::Database variant
        if let AppError::SqlxError(sqlx_err) = &e {
            if let sqlx::Error::Database(db_err) = sqlx_err {
                if db_err.is_unique_violation() {
                    return AppError::Conflict("Username or email already exist.".to_string());
                }
            }
        }
        e 
    })?;

    Ok(user)
}

pub async fn login_user(
    app_state: &web::Data<AppState>,
    req: LoginRequest
) -> Result<AuthResponse,AppError> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let user = find_by_username_or_email(&app_state.db, &req.email)
        .await
        .map_err(|_| AppError::NotFound("User not found".to_string()))?;
    if !password::verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = jwt::create_jwt(user.id, user.user_role.to_string(), &app_state.jwt_secret)?;

    Ok(AuthResponse {
        token,
        user_id: user.id.clone(),
        username: user.username.clone(),
        role: user.user_role.clone().to_string(),
    })
}




pub async fn logout_user(
    app_state: &web::Data<AppState>,
    claims: &Claims,
    token: &str,
) -> Result<(), AppError> {
    let mut conn = app_state.redis_pool.get().await
        .map_err(|e| {
            error!("Redis connection error: {}", e);
            AppError::RedisError(format!("Redis connection error: {}", e))
        })?;

    debug!("Attempting to blacklist token for user_id: {}", claims.user_id);
    
    let hashed_token = sha256_hash(token);
  
    let blacklist_key = format!("blacklisted_jwt:{}", hashed_token);
    
    // SET komutunu çalıştır
    match conn.set::<_, _, ()>(&blacklist_key, claims.user_id).await {
        Ok(_) => debug!("Token set in Redis blacklist"),
        Err(e) => {
            error!("Failed to set token in Redis: {}", e);
            return Err(AppError::RedisError(format!("Failed to set token in Redis: {}", e)));
        }
    }
    // Token'ın sona erme süresini hesaplayalım
    let now = chrono::Utc::now().timestamp() as usize;
    let ttl = claims.exp - now;
    
    // TTL ayarla (token süresi kadar)
    if ttl > 0 {
        match conn.expire::<_, bool>(&blacklist_key, ttl as i64).await {
            Ok(_) => debug!("Token TTL set to {} seconds", ttl),
            Err(e) => {
                error!("Failed to set token expiry in Redis: {}", e);
                return Err(AppError::RedisError(format!("Failed to set token expiry: {}", e)));
            }
        }
    }
    
    // İşlemleri kontrol edelim
    let exists: bool = conn.exists(&blacklist_key).await
        .map_err(|e| AppError::RedisError(format!("Redis error: {}", e)))?;
    
    if !exists {
        error!("Token was not added to blacklist!");
        return Err(AppError::InternalServerError("Token blacklisting failed".to_string()));
    }
    
    debug!("Successfully blacklisted token for user_id: {}", claims.user_id);
    Ok(())
}


