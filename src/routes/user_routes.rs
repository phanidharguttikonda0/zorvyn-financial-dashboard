use std::sync::Arc;
use axum::Router;
use axum::routing::{get, patch, post};
use crate::AppState;
use crate::controllers::user_controllers::{create_user, get_all_users, update_user};

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_user))
        .route("/{id}", patch(update_user))
        .route("/get-all-users", get(get_all_users))
        .route_layer(axum::middleware::from_fn(crate::middlewares::rbac::require_admin))
}