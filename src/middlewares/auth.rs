use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::authentication_models::Claims;
use crate::services::errors::ApplicationErrors;

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, ApplicationErrors> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            token
        } else {
            return Err(ApplicationErrors::Unauthorized("Invalid token format, must be Bearer <token>".to_string()));
        }
    } else {
        return Err(ApplicationErrors::Unauthorized("Missing authorization token header".to_string()));
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(t) => t,
        Err(err) => {
            if err.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                return Err(ApplicationErrors::TokenExpired("Token has expired. Please sign in again.".to_string()));
            } else {
                return Err(ApplicationErrors::Unauthorized("Invalid authorization token.".to_string()));
            }
        }
    };

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
