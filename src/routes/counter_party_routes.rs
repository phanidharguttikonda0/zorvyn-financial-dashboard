use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, post, put};
use crate::AppState;
use crate::controllers::counterparty_controllers::{create_party, delete_party, get_parties, update_party};

pub fn counter_party_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_party))
        .route("/:id", put(update_party)) // we need to cross check, where to use put and patch
       .route("/:id", delete(delete_party))
    .route("/", get(get_parties)) // returns all counter-parties
}