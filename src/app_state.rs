use sqlx::PgPool;
use deadpool_redis::Pool as RedisPool;
use std::sync::Arc;
use crate::config::AppConfig; 


#[derive(Clone)] 
pub struct AppState {
    pub db: PgPool, // PostgreSQL 
    pub redis_pool: RedisPool, // Redis 
    pub jwt_secret: String, // JWT 
    pub config: AppConfig, // App config
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        let db_pool = sqlx::PgPoolOptions::new()
            .max_connections(5)
            .connect_timeout(std::time::Duration::from_secs(5))
            .connect(&config.database_url)
            .await?;

        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await?;

        let redis_cfg = deadpool_redis::Config::from_url(config.redis_url.clone());
        let redis_pool = redis_cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;

        Ok(Arc::new(AppState {
            db: db_pool,
            redis_pool,
            jwt_secret: config.jwt_secret.clone(),
            config: config,
        }))
    }
}