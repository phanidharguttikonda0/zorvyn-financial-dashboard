use std::sync::Arc;
use axum::extract::{State, Query};
use axum::Json;
use chrono::Datelike;
use crate::AppState;
use crate::models::dashboard_models::{DashboardSummary, RecentFeed, CategoryAnalytics, TrendAnalytics, TrendQuery};
use crate::services::errors::ApplicationErrors;

pub async fn get_summary(State(app_state): State<Arc<AppState>>) -> Result<Json<DashboardSummary>, ApplicationErrors> {
    let summary = app_state.database.get_dashboard_summary().await
        .map_err(|e| ApplicationErrors::Database(e.to_string()))?;
    Ok(Json(summary))
}

pub async fn get_recent(State(app_state): State<Arc<AppState>>) -> Result<Json<RecentFeed>, ApplicationErrors> {
    let recent = app_state.database.get_recent_dashboard_transactions().await
        .map_err(|e| ApplicationErrors::Database(e.to_string()))?;
    Ok(Json(recent))
}

pub async fn get_by_category(State(app_state): State<Arc<AppState>>) -> Result<Json<CategoryAnalytics>, ApplicationErrors> {
    let analytics = app_state.database.get_dashboard_categories().await
        .map_err(|e| ApplicationErrors::Database(e.to_string()))?;
    Ok(Json(analytics))
}

pub async fn get_by_trends(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<TrendQuery>
) -> Result<Json<TrendAnalytics>, ApplicationErrors> {
    let year = query.year.unwrap_or(chrono::Utc::now().naive_utc().date().year());
    let trends = app_state.database.get_dashboard_trends(year).await
        .map_err(|e| ApplicationErrors::Database(e.to_string()))?;
    Ok(Json(trends))
}