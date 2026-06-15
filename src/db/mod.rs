use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> PgPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect DB")
}
