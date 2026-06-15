mod auth;
mod db;
mod handlers;
mod models;
mod routes;
mod templates;

use axum::Router;
use db::init_db;
use dotenvy::dotenv;
use routes::create_routes;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = init_db().await;

    let app = create_routes(pool);

    let port = env::var("APP_PORT").unwrap_or("3000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Server running on port {}", port);

    axum::serve(listener, app).await.unwrap();
}
