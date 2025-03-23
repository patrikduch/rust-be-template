use actix_web::web;
use crate::controllers::user_controller::UserController;
use crate::controllers::health_controller::HealthController;
use crate::controllers::ip_controller; // Changed from "ip" to "ip_controller"

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // User routes
    cfg.service(
        web::scope("/api/users")
            .route("", web::get().to(UserController::get_all_users))
            .route("", web::post().to(UserController::create_user))
            .route("/{id}", web::get().to(UserController::get_user_by_id))
            .route("/{id}", web::put().to(UserController::update_user))
            .route("/{id}", web::delete().to(UserController::delete_user))
    );
    
    // Health check route
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(HealthController::health_check))
    );

    cfg.service(
        web::scope("/api")
            .route("/ip", web::get().to(ip_controller::get_ip))
    );
}