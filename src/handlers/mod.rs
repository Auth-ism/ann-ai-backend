use actix_web::{web::{self, ServiceConfig}};

// Modülleri içe aktaralım
pub mod auth;
pub mod user;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Auth route'ları /api/auth altında topla
            .service(
                web::scope("/auth")
                    .service(auth::login)
                    .service(auth::register)
                    .service(auth::logout)
                    .service(auth::test_auth)
            )
            // User route'ları /api/users altında topla
            .service(
                web::scope("/users")
                    .service(user::get_user_by_id)
                    .service(user::get_user_by_email)
                    .service(user::list_all_users)
                    .service(user::search_users)
                    .service(user::get_recent_users)
                    .service(user::update_profile)
                    .service(user::change_user_password)
                    .service(user::update_role)
                    .service(user::deactivate_user)
                    .service(user::reactivate_user)
                    .service(user::verify_email)
            )
    );
}