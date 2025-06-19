use std::sync::Arc;

use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::get;
use deadpool_redis::redis::AsyncCommands;
use serde_json::json;
use crate::extension::auth::AuthenticatedUser;
use crate::utils::get_token_from_header::get_token_from_header;
use crate::{
    app_state::AppState, 
    error::AppError, 
    models::{
        dto::auth::{AuthResponse, LoginRequest, RegisterRequest}, 
        user::UserRole
    }, 
    services::auth as auth_service
};


#[post("/register")]
pub async fn register(
    app_state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let user = auth_service::register_user(&app_state, req.into_inner()).await?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "message": "User registered successfully",
        "data": {
            "user": user
        }
    })))
}
#[post("/login")]
pub async fn login(
    app_state: web::Data<AppState>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let user = auth_service::login_user(&app_state, req.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Login successful",
        "data": {
            "user": user
        }
    })))
}

#[post("/logout")]
pub async fn logout(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {

    auth_service::logout_user(&app_state, &auth_user.claims, &auth_user.token).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

#[get("/test-auth")]
pub async fn test_auth(
    auth_user: AuthenticatedUser,
) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Authentication successful",
        "user_id": auth_user.user_id,
        "role": auth_user.role
    }))
}

