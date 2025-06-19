use futures::Stream;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};



#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserUpdate {
    pub id: i32,
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(length(min = 3, max = 80))]
    pub full_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8))]
    pub password: Option<String>,
    #[validate(length(equal = 10))]
    pub phone_number: Option<String>,
}