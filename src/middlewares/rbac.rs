use axum::{
    extract::{Extension, Request},
    middleware::Next,
    response::Response,
};
use tracing::warn;
use crate::models::authentication_models::Claims;
use crate::services::errors::ApplicationErrors;

pub async fn block_viewer(
    Extension(claims): Extension<Claims>,
    req: Request,
    next: Next,
) -> Result<Response, ApplicationErrors> {
    if claims.role == "viewer" {
        warn!("Role 'viewer' denied access to endpoint");
        return Err(ApplicationErrors::Unauthorized("You don't have access to this route.".to_string()));
    }
    Ok(next.run(req).await)
}

pub async fn require_admin(
    Extension(claims): Extension<Claims>,
    req: Request,
    next: Next,
) -> Result<Response, ApplicationErrors> {
    if claims.role != "admin" {
        warn!("Role '{}' denied access to endpoint", claims.role);
        return Err(ApplicationErrors::Unauthorized("Admin privileges required.".to_string()));
    }
    Ok(next.run(req).await)
}
