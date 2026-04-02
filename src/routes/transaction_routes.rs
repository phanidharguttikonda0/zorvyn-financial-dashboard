use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, patch, post, put};
use crate::AppState;
use crate::controllers::transaction_controllers::{create_transaction, delete_transaction, get_transaction, get_transactions, update_transaction_patch, update_transaction_put};

pub fn transaction_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_transaction))
        .route("/:id", put(update_transaction_put))
        .route("/:id", delete(delete_transaction))
        .route("/:id", patch(update_transaction_patch))
        .route("/", get(get_transactions))
        .route("/:id", get(get_transaction))

}