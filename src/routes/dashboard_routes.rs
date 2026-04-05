use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::AppState;
use crate::controllers::dashboard_controllers::{get_by_category, get_by_trends, get_recent, get_summary};

pub fn dashboard_routes() -> Router<Arc<AppState>> {
    let viewer_routes = Router::new()
        .route("/summary", get(get_summary))
        .route("/recent", get(get_recent));

    let analyst_routes = Router::new()
        .route("/by-category", get(get_by_category))
        .route("/trends", get(get_by_trends))
        .route_layer(axum::middleware::from_fn(crate::middlewares::rbac::block_viewer));

    viewer_routes.merge(analyst_routes)
}