use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, post , get , patch};
use crate::AppState;
use crate::controllers::category_controllers::{create_category, delete_category, get_categories, get_category, update_category};

pub fn category_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_category).get(get_categories)) 
        .route("/{id}", get(get_category).patch(update_category).delete(delete_category))
        .route_layer(axum::middleware::from_fn(crate::middlewares::rbac::require_admin))
}