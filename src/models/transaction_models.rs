use serde::Deserialize;
use sqlx::types::chrono;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub amount: Option<f64>,
    pub transaction_date: Option<chrono::NaiveDate>,
    pub transaction_status: Option<String>,
    pub category_id: Option<i64>,
    pub counterparty_id: Option<i64>,
    pub created_by: Option<i64>, // nothing but admin id (user id)
}