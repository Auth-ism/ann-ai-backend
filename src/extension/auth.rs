use actix_web::{
dev::{Service, ServiceRequest, ServiceResponse, Transform}, 
web::Data, 
Error, FromRequest, HttpMessage
};
use futures::future::{LocalBoxFuture, Ready};
use std::{future::Future, rc::Rc};
use tracing::{debug, error, info};
use deadpool_redis::redis::AsyncCommands; // Redis komutları için

use crate::{
app_state::AppState, error::AppError, models::dto::auth::Claims, utils::{jwt, sha256::sha256_hash}
};
/// Authenticated user details extracted from validated JWT
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: String,
    pub claims: Claims,
    pub token: String
}

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token_res = req
            .headers()
            .get("Authorization")
            .ok_or(AppError::BadRequest(String::from("Authorization header was not passed.")))
            .and_then(|header_value| {
                header_value
                    .to_str()
                    .map(|v| v[7..].to_string())
                    .map_err(AppError::from)
            });
        let state_res = req.app_data::<Data<AppState>>()
            .ok_or(AppError::InternalServerError("AppState is missing in app.".to_string()))
            .map(|v| {
                v.clone()
            });
        Box::pin(async {
            let token = token_res?;
            let state = state_res?;
            let mut redis_con = match state.redis_pool.get().await {
                Ok(conn) => conn,
                Err(e) => return Err(AppError::RedisError(e.to_string())),
            };
           
            let claims = jwt::decode_jwt(&token, &state.jwt_secret)?;
            let token_hash = sha256_hash(&token);
            let blacklist_key = format!("blacklisted_jwt:{}", token_hash); // "token" yerine "jwt" kullan
            let is_blacklisted: bool = redis_con.exists(blacklist_key).await?;
            if is_blacklisted {
                return Err(AppError::Unauthorized("Token geçersiz kılındı (kara listede).".to_string()));
            }
            
            Ok(Self {
                role: claims.role.clone(),
                claims: claims.clone(),
                user_id: claims.user_id,
                token: token
            })
        })
    }
}


