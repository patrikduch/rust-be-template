// Re-export modules that will be needed in tests
// This makes your code accessible to integration tests

// Uncomment and adjust these based on your actual module structure
// pub mod routes;
// pub mod handlers;
// pub mod models;
// pub mod config;

// A minimal example if you don't have these modules yet:
pub mod routes {
    use actix_web::{web, HttpResponse, Responder};

    pub async fn index() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }
}

// Simple helper function you can test
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}