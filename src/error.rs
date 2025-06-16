use actix_web::{body::{BoxBody, MessageBody}, http, web::Json, HttpResponse, Responder};
use actix_web::ResponseError;
use serde::{Deserialize, Serialize};
use thiserror::Error;



#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status_code: u16,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request")]
    BadRequest(String),
    #[error("Unauthorized request")]
    Unauthorized(String),
    #[error("Forbidden request")]
    Forbidden(String),
    #[error("Not found")]
    NotFound(String),
    #[error("Unprocessable entity")]
    UnprocessableEntity(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            AppError::BadRequest(_) => http::StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => http::StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => http::StatusCode::FORBIDDEN,
            AppError::NotFound(_) => http::StatusCode::NOT_FOUND,
            AppError::UnprocessableEntity(_) => http::StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = ErrorResponse {
            message: self.to_string(),
            status_code: self.status_code().as_u16(),
        };
        eprint!("{:?}", body);
        HttpResponse::build(self.status_code()).json(body)
    }
}


 





