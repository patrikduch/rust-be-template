mod controllers;
mod models;
mod routes;
mod db;
mod services; 

use log::info;
use dotenv::dotenv;
use actix_web::{App, HttpServer, web};
use sqlx::{Pool, Postgres};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Initialize the database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let db_pool = web::Data::new(pool); // ✅ Wrap in `web::Data`

    info!("Starting server on 0.0.0.0:80...");

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone()) // ✅ Inject the database pool here
            .configure(routes::configure_routes) // ✅ No need to pass db_pool manually
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
