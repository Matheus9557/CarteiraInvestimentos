use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub balance: f64,
}
