mod controllers;
mod models;
mod routes;
use log::{info, warn, error};
use dotenv::dotenv;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:80");

    dotenv().ok(); // Load environment variables
    env_logger::init(); // Initialize logging
    
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}