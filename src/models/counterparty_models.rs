use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Party {
    pub id: Option<i64>,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub party_type: Option<String>, // 'vendor' | 'contractor' | 'employee' | 'client'
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}