use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

pub fn create_token(user_id: i32) -> String {
    let secret = env::var("JWT_SECRET").unwrap();

    let claims = Claims {
        sub: user_id,
        exp: 10000000000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).unwrap()
}
