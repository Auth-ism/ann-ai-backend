use sqlx::{PgPool, query_as};
use crate::models::user::{User, UserRole, CreateUserDto};
use crate::error::AppError;

pub async fn create(
    db: &PgPool, 
    user_data: &CreateUserDto,
    password_hash: &str,
    role: UserRole
) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
         r#"
    INSERT INTO user_info (username, full_name, email, password_hash, phone_number, user_role)
    VALUES ($1, $2, $3, $4, $5, $6)
    RETURNING id, username, full_name, email, password_hash, phone_number, 
              user_role as "user_role: UserRole", token_balance, 
              created_at, updated_at,
              null as "subscription_expries?", 
              false as "email_verified", 
              false as "phone_verified",
              null as "last_login?",
              true as "is_active"
    "#,
      user_data.username,
      user_data.full_name,
      user_data.email,
      password_hash,
      user_data.phone_number,
    role as UserRole
    )
    .fetch_one(db)
    .await?;

    Ok(user)
}


pub async fn find_by_username_or_email(db: &PgPool, identifier: &str) -> Result<Option<User>, AppError> {
    let user = query_as!(
        User,
        r#"
        SELECT
            id, username, full_name, email, password_hash, phone_number,
            user_role as "user_role: UserRole", token_balance, created_at, updated_at,
            subscription_expries, email_verified, phone_verified, last_login, is_active
        FROM 
            user_info
        WHERE
            username = $1 OR email = $2
        "#,
        identifier,
        identifier
    )
    .fetch_optional(db)
    .await?;
    Ok(user)
}

pub async fn find_by_id(db: &PgPool, user_id: Uuid) -> Result<Option<User>, AppError> {
    let user = query_as!(
        User,
        r#"
        SELECT
            id, username, full_name, email, password_hash, phone_number,
            user_role as "user_role: UserRole", token_balance, created_at, updated_at,
            subscription_expries, email_verified, phone_verified, last_login, is_active
        FROM 
            user_info
        WHERE
            id = $1
        "#,
        user_id
    )
    .fetch_optional(db)
    .await?;
    Ok(user)
}