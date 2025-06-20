use crate::error::AppError;
use crate::models::user::{User, UserRole,UserSchema};
use sqlx::PgPool;
use bigdecimal::{BigDecimal, ToPrimitive};
use uuid::Uuid;

pub async fn create(
    db: &PgPool,
    user_data: &UserSchema,
    password_hash: &str,
    role: UserRole,
) -> Result<User, AppError> {
    let user_schema = sqlx::query_as!(
        UserSchema,
        r#"
        INSERT INTO user_info (username, full_name, email, password_hash, phone_number, user_role)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, username, full_name, email, password_hash, phone_number,
                  token_balance, user_role as "user_role: UserRole",
                  subscription_expries, email_verified, phone_verified, 
                  last_login, is_active, created_at, updated_at
        "#,
        user_data.username,
        user_data.full_name,
        user_data.email,
        password_hash,
        user_data.phone_number,
        role as _
    )
    .fetch_one(db)
    .await
    .map_err(AppError::from)?;

    let user = User::try_from(user_schema)?;
    Ok(user)
}



pub async fn find_by_username_or_email(
     db: &PgPool,
     mail_or_id: &str
) -> Result<User,AppError> {
    let user_schema = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT id, username, full_name, email, password_hash, phone_number, token_balance, user_role as "user_role: UserRole",
               subscription_expries, email_verified, phone_verified, last_login, is_active, created_at, updated_at
        FROM user_info
        WHERE username = $1 OR email = $2
        "#,
        mail_or_id,
        mail_or_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::db_error(&e.to_string()))?;

    let user = User::try_from(user_schema)?;
    Ok(user)
}
