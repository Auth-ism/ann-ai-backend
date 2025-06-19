use actix_web::{web::{self, ServiceConfig}, HttpResponse};

// Modülleri içe aktaralım
pub mod auth;
pub mod users;  // users modülünü ekleyin

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                // Tüm auth routeları /api/auth altında topla
                web::scope("/auth")
                    .service(auth::login)
                    .service(auth::register)
                    .service(auth::logout)
                    .service(auth::test_auth)
            )


            
    );


}