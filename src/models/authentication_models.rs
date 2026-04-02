use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignIn {
    pub email: String,
    pub password: String
}