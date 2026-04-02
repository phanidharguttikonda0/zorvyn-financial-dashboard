use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, post , get , put};
use crate::AppState;
use crate::controllers::category_controllers::{create_category, delete_category, get_categories, get_category, update_category};

pub fn category_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:id", post(create_category))
        .route("/", get(get_categories)) // get's all categories, we will implement cursor pagination
        .route("/:id", get(get_category)) // get's single categories
        .route("/:id", put(update_category))
        .route("/:id", delete(delete_category))
}