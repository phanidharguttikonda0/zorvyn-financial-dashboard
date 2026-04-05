use std::sync::Arc;
use tower_http::cors::CorsLayer;
use axum::http::StatusCode;
use axum::response::IntoResponse;
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
    pub database: DBService,
    pub rate_limiter: Arc<tokio::sync::Mutex<std::collections::HashMap<std::net::IpAddr, (usize, tokio::time::Instant)>>>,
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
    dotenv::dotenv().ok();

    let tcp_connection = TcpListener::bind("0.0.0.0:7878").await.expect("Can't listen, something running on that port") ;

    axum::serve(tcp_connection, routes().await.into_make_service_with_connect_info::<std::net::SocketAddr>()).await.expect("Can't launch server");
}

async fn routes() -> Router {
    let database = DBService::new().await;
    database.check_admin_and_init().await;

    let state = Arc::new(AppState {
        database,
        rate_limiter: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
    });

    let protected_routes = Router::new()
        .nest("/transaction", transaction_routes())
        .nest("/categories", category_routes())
        .nest("/counter-parties", counter_party_routes())
        .nest("/dashboard", dashboard_routes())
        .nest("/users", user_routes())
        .route_layer(axum::middleware::from_fn(crate::middlewares::auth::auth_middleware));

    let cors = CorsLayer::permissive();

    Router::new().route("/health", get(health_check_point))
        .route("/docs", get(render_docs))
        .nest("/authentication", authentication_routes())
        .merge(protected_routes)
        .layer(cors)
        .route_layer(axum::middleware::from_fn_with_state(state.clone(), crate::middlewares::rate_limit::rate_limit_middleware))
        .with_state(state)
}

async fn render_docs() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../public/index.html"))
}

async fn health_check_point() -> impl IntoResponse {
    StatusCode::OK
}