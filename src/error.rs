use actix_web::{
    body::BoxBody,
    http::{self, header::ToStrError, StatusCode},
    HttpResponse, ResponseError,
};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use tracing::{debug, error};
use deadpool::managed::PoolError;
use deadpool_redis::redis::RedisError;

// Define the JwtTokenError trait
pub trait JwtTokenError {
    fn token_invalid() -> Self;
    fn token_expired() -> Self;
    fn token_missing() -> Self;
    fn from_jwt_error(err: jsonwebtoken::errors::Error) -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub error: String,
    pub message: String,
    //#[serde(skip_serializing_if = "Option::is_none")]
    // pub details: Option<String>,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.status_code, self.error, self.message)
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Database Error: {0}")]
    DbError(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("Argon2 Error: {0}")]
    Argon2Error(String),

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    RedisCmdError(#[from] redis::RedisError),

    #[error(transparent)]
    ToStrError(#[from] ToStrError)
}

// Implement JwtTokenError for AppError
impl JwtTokenError for AppError {
    fn token_invalid() -> Self {
        Self::Unauthorized("Invalid authentication token".to_string())
    }

    fn token_expired() -> Self {
        Self::Unauthorized("Authentication token expired".to_string())
    }

    fn token_missing() -> Self {
        Self::Unauthorized("Authentication token is missing".to_string())
    }

    fn from_jwt_error(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            ErrorKind::ExpiredSignature => Self::token_expired(),
            ErrorKind::InvalidToken => Self::token_invalid(),
            ErrorKind::InvalidSignature => {
                Self::Unauthorized("Invalid token signature".to_string())
            }
            ErrorKind::InvalidIssuer => Self::Unauthorized("Invalid token issuer".to_string()),
            ErrorKind::InvalidAudience => Self::Unauthorized("Invalid token audience".to_string()),
            ErrorKind::InvalidSubject => Self::Unauthorized("Invalid token subject".to_string()),
            _ => Self::Unauthorized(format!("Token error: {}", err)),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::SqlxError(e) => match e {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status = self.status_code();

        // let details = match self {
        //     AppError::SqlxError(e) => Some(e.to_string()),
        //     AppError::JwtError(e) => Some(e.to_string()),
        //     AppError::RedisError(e) => Some(e.to_string()),
        //     AppError::RedisCmdError(e) => Some(e.to_string()),
        //     _ => None,
        // };

        if status.is_server_error() {
            error!(?self, status = status.as_u16(), "Server error occurred");
        } else {
            debug!(?self, status = status.as_u16(), "Client error occurred");
        }

        let response = ErrorResponse {
            status_code: status.as_u16(),
            error: status.canonical_reason().unwrap_or("Unknown").to_string(),
            message: self.to_string(),
            //details: if cfg!(debug_assertions) { details } else { None },
        };

        HttpResponse::build(status).json(response)
    }
}

impl AppError {
    pub fn not_found(resource: &str) -> Self {
        Self::NotFound(format!("Resource not found: {}", resource))
    }

    pub fn unauthorized(reason: &str) -> Self {
        Self::Unauthorized(format!("Unauthorized access: {}", reason))
    }

    pub fn forbidden(reason: &str) -> Self {
        Self::Forbidden(format!("Access forbidden: {}", reason))
    }

    pub fn bad_request(message: &str) -> Self {
        Self::BadRequest(format!("Bad request: {}", message))
    }

    pub fn conflict(message: &str) -> Self {
        Self::Conflict(format!("Conflict occurred: {}", message))
    }

    pub fn internal_error(message: &str) -> Self {
        Self::InternalServerError(format!("Internal server error: {}", message))
    }

    pub fn validation_error(message: &str) -> Self {
        Self::ValidationError(format!("Validation error: {}", message))
    }

    pub fn db_error(message: &str) -> Self {
        Self::DbError(format!("Database error: {}", message))
    }
}

// Implementation for the Redis pool error
impl From<PoolError<RedisError>> for AppError {
    fn from(err: PoolError<RedisError>) -> Self {
        AppError::internal_error(&format!("Redis pool error: {}", err))
    }
}

// You already have this implementation for RedisError itself
impl From<RedisError> for AppError {
    fn from(err: RedisError) -> Self {
        AppError::internal_error(&format!("Redis error: {}", err))
    }
}