use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Transaction {
    pub id: Option<i64>,
    pub amount: Option<f64>,
    pub transaction_date: Option<chrono::NaiveDate>,
    #[serde(rename = "status")]
    pub transaction_status: Option<String>,
    pub category_id: Option<i64>,
    pub counterparty_id: Option<i64>,
    pub created_by: Option<i64>, // admin id (user id)
}