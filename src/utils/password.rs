use actix_web::{body::MessageBody, App};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::{error::AppError, result::AppResult};

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Argon2Error((e.to_string())))
        .map(|hash| hash.to_string())
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let parsed_hash = argon2::password_hash::PasswordHash::new(hashed_password)
        .map_err(|e| AppError::Argon2Error((e.to_string())))?;

    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
