use std::sync::Arc;
use tokio::time::Instant;
use axum::{
    extract::{State, Request, ConnectInfo},
    middleware::Next,
    response::Response,
};
use tracing::warn;
use crate::AppState;
use crate::services::errors::ApplicationErrors;

pub async fn rate_limit_middleware(
    State(app_state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, ApplicationErrors> {
    let ip = addr.ip();
    let mut limiter = app_state.rate_limiter.lock().await;

    let now = Instant::now();
    let entry = limiter.entry(ip).or_insert((0, now));

    if now.duration_since(entry.1).as_secs() > 60 {
        entry.0 = 0;
        entry.1 = now;
    }

    if entry.0 >= 25 {
        warn!("Rate limit exceeded for IP: {}", ip);
        return Err(ApplicationErrors::RateLimitExceeded("Too many requests from this IP. Please try again after a minute.".to_string()));
    }

    entry.0 += 1;
    drop(limiter);

    Ok(next.run(req).await)
}
