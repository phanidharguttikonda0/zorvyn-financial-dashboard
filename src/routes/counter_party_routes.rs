use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, patch, post};
use crate::AppState;
use crate::controllers::counterparty_controllers::{create_party, delete_party, get_parties, update_party};

pub fn counter_party_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_party).get(get_parties))
        .route("/{id}", patch(update_party).delete(delete_party))
        .route_layer(axum::middleware::from_fn(crate::middlewares::rbac::require_admin))
}