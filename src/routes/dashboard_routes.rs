use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::AppState;
use crate::controllers::dashboard_controllers::{get_by_category, get_by_trends, get_recent, get_summary};

pub fn dashboard_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/by-category", get(get_by_category))
        .route("/trends", get(get_by_trends))
        .route("/summary", get(get_summary))
        .route("/recent", get(get_recent))
}