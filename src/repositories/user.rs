use sqlx::{PgPool};
use chrono::Utc;
use crate::models::dto::user::UserUpdate;
use crate::models::user::{User, UserSchema};
use crate::error::AppError;

pub async fn find_user_by_email(db: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let user_schema = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT * FROM user_info WHERE email = $1
        "#,
        email
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;
    let user = user_schema.map(|schema| User::try_from(schema)).transpose()?;

    Ok(user)
}

pub async fn find_user_by_id(db: &PgPool, user_id: i32) -> Result<Option<User>, AppError> {
    let user_schema = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT * FROM user_info WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    let user = user_schema.map(|schema| User::try_from(schema)).transpose()?;

    Ok(user)
}

pub async fn update_last_login(db: &PgPool, user_id: i32) -> Result<(), AppError> {
    let now = Utc::now();

    sqlx::query!(
        r#"
        UPDATE user_info SET last_login = $1 WHERE id = $2
        "#,
        now,
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok(())
}


// Get all users (with pagination)
pub async fn get_users(
    db: &PgPool,
    limit: i64,
    offset: i64
) -> Result<Vec<User>, AppError> {
    let user_schemas = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT * FROM user_info
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    let users = user_schemas
        .into_iter()
        .map(|schema| User::try_from(schema))
        .collect::<Result<Vec<User>, AppError>>()?;

    Ok(users)
}

// Get single user by ID
pub async fn get_user_by_id(
    db: &PgPool,
    user_id: i32
) -> Result<Option<User>, AppError> {
    let user_schema = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT * FROM user_info WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    let user = user_schema.map(|schema| User::try_from(schema)).transpose()?;

    Ok(user)
}

// Get user by username
pub async fn get_user_by_username(
    db: &PgPool,
    username: &str
) -> Result<Option<User>, AppError> {
    let user_schema = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT * FROM user_info WHERE username = $1
        "#,
        username
    )
    .fetch_optional(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    let user = user_schema.map(|schema| User::try_from(schema)).transpose()?;

    Ok(user)
}

// Update user profile
pub async fn update_user(
    db: &PgPool,
    update: &UserUpdate,
) -> Result<User, AppError> {
    let user_schema = sqlx::query_as!(
        UserSchema,
        r#"
        UPDATE user_info
        SET 
            full_name = COALESCE($1, full_name),
            username = COALESCE($2, username),
            phone_number = COALESCE($3, phone_number),
            updated_at = $4
        WHERE id = $5
        RETURNING *
        "#,
        update.full_name,
        update.username,
        update.phone_number,
        Utc::now(),
        update.id
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    User::try_from(user_schema)
}

// Change user password
pub async fn update_password(
    db: &PgPool,
    user_id: i32,
    password_hash: &str
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE user_info
        SET password_hash = $1, updated_at = $2
        WHERE id = $3
        "#,
        password_hash,
        Utc::now(),
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok(())
}

// Deactivate user (soft delete)
pub async fn deactivate_user(
    db: &PgPool,
    user_id: i32
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE user_info
        SET is_active = false, updated_at = $1
        WHERE id = $2
        "#,
        Utc::now(),
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok(())
}

// Reactivate user
pub async fn reactivate_user(
    db: &PgPool,
    user_id: i32
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE user_info
        SET is_active = true, updated_at = $1
        WHERE id = $2
        "#,
        Utc::now(),
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok(())
}

// Update user role (admin only)
pub async fn update_user_role(
    db: &PgPool,
    user_id: i32,
    role: &str
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE user_info
        SET user_role = $1, updated_at = $2
        WHERE id = $3
        "#,
        role,
        Utc::now(),
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok(())
}

// Verify user's email
pub async fn set_email_verified(
    db: &PgPool,
    user_id: i32,
    verified: bool
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        UPDATE user_info
        SET email_verified = $1, updated_at = $2
        WHERE id = $3
        "#,
        verified,
        Utc::now(),
        user_id
    )
    .execute(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    Ok(())
}

// Count total users (for admin dashboard)
pub async fn count_users(db: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM user_info
        "#
    )
    .fetch_one(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?
    .count
    .unwrap_or(0);

    Ok(count)
}

// Search users
pub async fn search_users(
    db: &PgPool,
    query: &str,
    limit: i64,
    offset: i64
) -> Result<Vec<User>, AppError> {
    let search = format!("%{}%", query);
    
    let user_schemas = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT * FROM user_info
        WHERE username ILIKE $1
           OR email ILIKE $1
           OR full_name ILIKE $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        search,
        limit,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    let users = user_schemas
        .into_iter()
        .map(|schema| User::try_from(schema))
        .collect::<Result<Vec<User>, AppError>>()?;
    Ok(users)

}

pub async fn get_recent_users(
    db: &PgPool,
    days: i64,
    limit: i64
) -> Result<Vec<User>, AppError> {
    let user_schemas = sqlx::query_as!(
        UserSchema,
        r#"
        SELECT * FROM user_info
        WHERE created_at > NOW() - ($1 || ' days')::INTERVAL
        ORDER BY created_at DESC
        LIMIT $2
        "#,
        days.to_string(),
        limit
    )
    .fetch_all(db)
    .await
    .map_err(|e| AppError::SqlxError(e))?;

    let users = user_schemas
        .into_iter()
        .map(|schema| User::try_from(schema))
        .collect::<Result<Vec<User>, AppError>>()?;

    Ok(users)
}   