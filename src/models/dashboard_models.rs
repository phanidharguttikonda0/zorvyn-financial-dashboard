use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RecentTransaction {
    pub id: i64,
    pub amount: f64,
    pub date: String,
    pub category: String,
    pub r#type: String,
    pub counterparty: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct RecentFeed {
    pub transactions: Vec<RecentTransaction>,
}

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    pub total_income: f64,
    pub total_expenses: f64,
    pub net_balance: f64,
    pub total_transactions: i64,
}

#[derive(Debug, Serialize)]
pub struct CategoryTotal {
    pub category: String,
    pub total: f64,
}

#[derive(Debug, Serialize)]
pub struct CategoryAnalytics {
    pub expense_categories: Vec<CategoryTotal>,
    pub income_categories: Vec<CategoryTotal>,
}

#[derive(Debug, Serialize)]
pub struct MonthTrend {
    pub month: String,
    pub income: f64,
    pub expenses: f64,
    pub net: f64,
}

#[derive(Debug, Serialize)]
pub struct TrendAnalytics {
    pub year: i32,
    pub months: Vec<MonthTrend>,
}

#[derive(Debug, Deserialize)]
pub struct TrendQuery {
    pub year: Option<i32>,
}
