use actix_web::{HttpResponse, Responder};

pub struct HealthController;

impl HealthController {
    pub async fn health_check() -> impl Responder {
        HttpResponse::Ok().json("Service is healthy")
    }
}