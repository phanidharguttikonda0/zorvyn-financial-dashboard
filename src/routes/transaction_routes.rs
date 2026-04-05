use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, patch, post};
use crate::AppState;
use crate::controllers::transaction_controllers::{create_transaction, delete_transaction, get_transaction, get_transactions, update_transaction};

pub fn transaction_routes() -> Router<Arc<AppState>> {
    let analyst_routes = Router::new()
        .route("/", get(get_transactions))
        .route("/{id}", get(get_transaction))
        .route_layer(axum::middleware::from_fn(crate::middlewares::rbac::block_viewer));

    let admin_routes = Router::new()
        .route("/", post(create_transaction))
        .route("/{id}", delete(delete_transaction))
        .route("/{id}", patch(update_transaction))
        .route_layer(axum::middleware::from_fn(crate::middlewares::rbac::require_admin));

    analyst_routes.merge(admin_routes)
}