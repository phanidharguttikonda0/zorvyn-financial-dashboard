use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::AppState;
use crate::controllers::authentication_controllers::sign_in;

pub fn authentication_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sign-in", post(sign_in))
}