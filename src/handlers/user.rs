use actix_web::{web, get, put, post, delete, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::{
    app_state::AppState, error::AppError, extension::auth::AuthenticatedUser, models::{dto::user::{PaginationQuery, PasswordChangeRequest, RecentUsersQuery, UserUpdate}, user::UserRole}, result::AppResult, services::user
};
use log::{debug, info};

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: UserRole,
}



// ===== User Retrieval =====

#[get("/{id}")]
pub async fn get_user_by_id(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires authentication
    path: web::Path<i32>
) -> AppResult<impl Responder> {
    // Only admins or the user themselves can access this endpoint
    let user_id = path.into_inner();
    if auth_user.user_id != user_id && auth_user.role != "admin" {
        return Err(AppError::Forbidden("Not authorized to access this user".to_string()));
    }
    
    let result = user::find_user_id(&app_state, web::Path::from(user_id)).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/email/{email}")]
pub async fn get_user_by_email(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    path: web::Path<String>
) -> AppResult<impl Responder> {
    // Only admins can search by email (for security)
    if auth_user.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let result = user::find_user_mail(&app_state, path).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("")]
pub async fn list_all_users(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    query: web::Query<PaginationQuery>
) -> AppResult<impl Responder> {
    if auth_user.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let (users, total) = user::list_users(
        &app_state, 
        query.page, 
        query.page_size
    ).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "users": users,
        "total": total,
        "page": query.page,
        "page_size": query.page_size
    })))
}

#[get("/search/{query}")]
pub async fn search_users(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    path: web::Path<String>,
    query: web::Query<PaginationQuery>
) -> AppResult<impl Responder> {
    // Only admins can search users
    if auth_user.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let search_query = path.into_inner();
    let users = user::search_users_service(
        &app_state,
        &search_query,
        query.page,
        query.page_size
    ).await?;
    
    Ok(HttpResponse::Ok().json(users))
}

#[get("/recent")]
pub async fn get_recent_users(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    query: web::Query<RecentUsersQuery>
) -> AppResult<impl Responder> {
    // Only admins can view recent users
    if auth_user.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let users = user::get_recent_users_service(
        &app_state,
        query.days,
        query.limit
    ).await?;
    
    Ok(HttpResponse::Ok().json(users))
}

// ===== User Profile =====

#[put("")]
pub async fn update_profile(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    req: web::Json<UserUpdate>
) -> AppResult<impl Responder> {
    // Users can only update their own profile unless they're admin
    if req.id != auth_user.user_id && auth_user.role != "admin" {
        return Err(AppError::Forbidden("Not authorized to update this user".to_string()));
    }
    
    let updated_user = user::update_user_service(&app_state, req.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_user))
}


#[put("/password")]
pub async fn change_user_password(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    req: web::Json<PasswordChangeRequest>
) -> AppResult<impl Responder> {
    user::change_password(
        &app_state,
        auth_user.user_id,
        &req.current_password,
        &req.new_password
    ).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Password updated successfully"})))
}

// ===== Admin Actions =====
#[put("/{id}/role")]

pub async fn update_role(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    path: web::Path<i32>,
    req: web::Json<UpdateRoleRequest>
) -> AppResult<impl Responder> {
    // Only admins can change roles
    if auth_user.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let user_id = path.into_inner();
    user::update_user_role_service(&app_state, user_id, req.role.clone()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "User role updated successfully"})))
}

#[delete("/{id}")]
pub async fn deactivate_user(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    path: web::Path<i32>
) -> AppResult<impl Responder> {
    // Only admins can deactivate users
    if auth_user.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let user_id = path.into_inner();
    // Prevent self-deactivation
    if user_id == auth_user.user_id {
        return Err(AppError::BadRequest("Cannot deactivate your own account".to_string()));
    }
    
    user::deactivate_user_service(&app_state, user_id).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "User deactivated successfully"})))
}

#[post("/{id}/activate")]
pub async fn reactivate_user(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    path: web::Path<i32>
) -> AppResult<impl Responder> {
    // Only admins can reactivate users
    if auth_user.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let user_id = path.into_inner();
    user::reactivate_user_service(&app_state, user_id).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "User reactivated successfully"})))
}

#[post("/{id}/verify-email")]
pub async fn verify_email(
    app_state: web::Data<AppState>,
    auth_user: AuthenticatedUser,  // Requires auth
    path: web::Path<i32>
) -> AppResult<impl Responder> {
    // Only admins or the user themselves can verify email
    let user_id = path.into_inner();
    if auth_user.user_id != user_id && auth_user.role != "admin" {
        return Err(AppError::Forbidden("Not authorized for this action".to_string()));
    }
    
    user::mark_email_verified(&app_state, user_id).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Email verified successfully"})))
}
