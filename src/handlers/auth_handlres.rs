use axum::{Json};
use serde::Deserialize;
use bcrypt::{hash, verify};
use sqlx::PgPool;
use crate::auth::jwt::create_token;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
    pool: axum::extract::Extension<PgPool>,
) -> String {

    let user = sqlx::query!(
        "SELECT id, password FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool.0)
    .await
    .unwrap();

    if verify(payload.password, &user.password).unwrap() {
        return create_token(user.id);
    }

    "invalid credentials".to_string()
}
