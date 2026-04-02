use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;


#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String
}

#[derive(Debug)]
pub enum ApplicationErrors {
    Database(String),
    Validation(String),
    NotFound(String),
}

impl IntoResponse for ApplicationErrors {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApplicationErrors::Database(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ApplicationErrors::Validation(message) => (StatusCode::BAD_REQUEST, message),
            ApplicationErrors::NotFound(message) => (StatusCode::NOT_FOUND, message),
        } ;
        let body = Json(ApiError {
            error: status.to_string(),
            message: message.to_string()
        }) ;

        (status, body).into_response()
    }
}