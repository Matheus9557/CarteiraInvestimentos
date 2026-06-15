use axum::{Router, routing::get, routing::post};
use sqlx::PgPool;

use crate::handlers::{
    auth_handler::login,
    wallet_handler::get_balance,
};

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/wallet", get(get_balance))
        .layer(axum::Extension(pool))
}
