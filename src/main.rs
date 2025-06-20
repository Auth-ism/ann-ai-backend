mod app_state;
mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod services;
mod utils;
mod extension;
mod result;

use actix_web::{middleware, web::{self, Data}, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use tracing::{info, error};

use crate::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = run_server().await {
        eprintln!("Uygulama başlatılırken hata: {}", e);
        std::process::exit(1);
    }
    Ok(())
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Loglama sistemini yapılandır
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(true)
        .init();
    
    info!("Uygulama başlatılıyor...");

    let config = config::AppConfig::from_env()?;
    info!("Yapılandırma yüklendi");
    
    println!("AppState oluşturuluyor... Veritabanı bağlantısı kuruluyor...");
    let app_state = Data::new(AppState::new(config).await?);
    info!("AppState başarıyla oluşturuldu");
    
    let bind_address = format!("{}:{}", "0.0.0.0", 3000);
    info!("Sunucu {} adresinde başlatılıyor", bind_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
            
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            // handlers/mod.rs'deki configure fonksiyonunu kullan
            .configure(handlers::configure)
            // Sağlık kontrolü ana dosyada olsun
            .route("/health", web::get().to(|| async { 
                HttpResponse::Ok().body("OK") 
            }))
    })
    .workers(num_cpus::get())
    .bind(&bind_address)?
    .run()
    .await?;

    info!("Sunucu kapatılıyor");
    Ok(())
}