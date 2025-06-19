use crate::config::AppConfig;
use crate::error::AppError;
use deadpool_redis::Pool as RedisPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc; // Add missing import

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,            // PostgreSQL
    pub redis_pool: RedisPool, // Redis
    pub jwt_secret: String,    // JWT
    pub config: AppConfig,     // App config

}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&config.database_url)
            .await?;
        println!("Migration BAŞLIYOR");
        sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();
        println!("Migration BİTTİ");

        let redis_cfg = deadpool_redis::Config::from_url(&config.redis_url);
        let redis_pool = redis_cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap();

        Ok(AppState {
            db: db_pool,
            redis_pool,
            jwt_secret: config.jwt_secret.clone(),
            config, // Simplified field assignment
        })
    }
}
