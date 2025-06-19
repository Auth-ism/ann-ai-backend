use std::default;

use crate::{error::AppError, models::dto::auth::Claims};
use actix_web::http::header::TryIntoHeaderValue;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

pub fn create_jwt(user_id: i32, user_role: String, secret: &str) -> Result<String, AppError> {
    let expiration = Utc::now() + Duration::hours(24);
    let iat = Utc::now().timestamp() as usize;

    let claim = Claims {
        user_id,
        role: user_role,
        exp: expiration.timestamp() as usize,
        iat,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    encode(&header, &claim, &encoding_key).map_err(|e| AppError::JwtError((e)))
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::default();
    validation.validate_exp = true;
    validation.validate_nbf = false;

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| AppError::JwtError((e)))
}
