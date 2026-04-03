use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Category {
    pub id: Option<i64>,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub category_type: Option<String>, // Should match 'income' | 'expenses'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}