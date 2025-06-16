
use futures::Stream;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
#[derive(Debug, Deserialize, Serialize, Validate)]


pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(equal = 10))]
    pub phone_number: String

}


#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub email: String, 
    #[validate(length(min = 1))]
    pub password: String,
}


#[derive(Debug,Deserialize,Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: Uuid,
    pub username: String,
    pub role: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize, 
    pub iat: usize, 
    pub role: String, 
}

