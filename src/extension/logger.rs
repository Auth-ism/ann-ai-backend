// src/middleware/logger.rs
use actix_web::middleware::Logger;

pub fn new_logger() -> Logger {
    Logger::default()
        .log_target("http_requests") // Logların gönderileceği hedef
}