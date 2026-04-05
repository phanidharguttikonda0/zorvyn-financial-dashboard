use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::AppState;
use crate::controllers::counterparty_controllers::{create_party, delete_party, get_parties, get_party, update_party};

pub fn counter_party_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_party).get(get_parties))
        .route("/{id}", get(get_party).patch(update_party).delete(delete_party))
        .route_layer(axum::middleware::from_fn(crate::middlewares::rbac::require_admin))
}