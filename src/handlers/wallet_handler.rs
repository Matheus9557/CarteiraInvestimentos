use axum::extract::Extension;
use sqlx::PgPool;

pub async fn get_balance(
    Extension(pool): Extension<PgPool>,
) -> String {

    let row = sqlx::query!("SELECT balance FROM wallet LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    format!("Balance: {}", row.balance)
}
