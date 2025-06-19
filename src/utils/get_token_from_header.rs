use actix_web::HttpRequest;

use crate::error::AppError;

pub fn get_token_from_header(req: &HttpRequest) -> Result<&str, AppError> {
    let token = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::bad_request("Missing or invalid authorization header"))?;
    
    Ok(token)
}