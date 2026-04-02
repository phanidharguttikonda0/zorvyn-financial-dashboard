use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, patch, post};
use crate::AppState;
use crate::controllers::transaction_controllers::{create_transaction, delete_transaction, get_transaction, get_transactions, update_transaction};

pub fn transaction_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_transaction))
        .route("/:id", delete(delete_transaction))
        .route("/:id", patch(update_transaction))
        .route("/", get(get_transactions))
        .route("/:id", get(get_transaction))

}