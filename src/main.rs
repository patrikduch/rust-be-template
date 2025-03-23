mod controllers;
mod models;
mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}