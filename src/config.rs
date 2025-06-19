use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub redis_url: String,
    pub host: String,
    pub port: u16,
    pub admin_registration_code: String
}

impl AppConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();
        envy::from_env()
    }
}
