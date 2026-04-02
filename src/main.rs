use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use tracing::info;
use tracing_appender::{rolling, non_blocking};
use tracing_subscriber::EnvFilter;
use crate::routes::authentication_routes::authentication_routes;
use crate::routes::category_routes::category_routes;
use crate::routes::counter_party_routes::counter_party_routes;
use crate::routes::dashboard_routes::dashboard_routes;
use crate::routes::transaction_routes::transaction_routes;
use crate::routes::user_routes::user_routes;
use crate::services::db::DBService;

mod models;
mod middlewares;
mod controllers;
mod services;
mod routes;





#[derive(Debug, Clone)]
pub struct AppState {
    database: DBService
}


#[tokio::main]
async fn main() {

    // Async Loging, creates a log file every day , we can use hourly , to use new log file every hour
    let file_appender = rolling::daily("logs", "app.log");

    let (non_blocking, _guard) = non_blocking(file_appender);

    // Subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    info!("Loading the environment...");
    dotenv::dotenv().ok().expect("Failed to load .env file");

    let tcp_connection = TcpListener::bind("127.0.0.1:7878").await.expect("Can't listen, something running on that port") ;

    axum::serve(tcp_connection, routes().await).await.expect("Can't launch server");
}

async fn routes() -> Router {
    let state = Arc::new(AppState {
        database: DBService::new().await
    }) ;
    Router::new().route("/health", get(health_check_point))
        .nest("/authentication", authentication_routes())
        .nest("/transaction", transaction_routes())
        .nest("/categories", category_routes())
        .nest("/counter-parties", counter_party_routes())
        .nest("/dashboard", dashboard_routes())
        .nest("/users", user_routes())
        .with_state(state)
}

async fn health_check_point() -> &'static str {
    "server running successfully!"
}