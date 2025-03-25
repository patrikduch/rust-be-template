use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;
use dotenv::dotenv;

pub async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok(); // Load environment variables

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool")
}