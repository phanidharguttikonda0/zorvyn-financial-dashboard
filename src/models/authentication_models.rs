use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct SignIn {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AuthorizationToken {
    pub access_token: String,
    pub token_type: String, // Bearer
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // email
    pub name: String,
    pub role: String,
    pub exp: usize,    // expiry timestamp
}