use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use crate::models::authentication_models::{Claims, UserInfo};

pub fn generate_jwt_token(user_info: UserInfo) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_info.email,
        name: user_info.name,
        role: user_info.role,
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
        .expect("Failed to create token")
}


pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let hashed_input = hash_password(password);
    hashed_input == stored_hash
}

use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(password.as_bytes());

    let result = hasher.finalize(); // 32 bytes

    general_purpose::STANDARD.encode(result) // 44 chars
}