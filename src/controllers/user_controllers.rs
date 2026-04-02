use std::sync::Arc;
use axum::extract::{Path, State, Query};
use axum::{Form, Json};
use crate::AppState;
use crate::models::user_models::User;
use crate::services::errors::{ApiResponse, ApplicationErrors};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    last_id: Option<i64>,
    limit: Option<i64>,
}

pub async fn create_user(
    State(app_state): State<Arc<AppState>>, 
    Form(user_details): Form<User>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    
    match app_state.database.create_user(user_details).await {
        Ok(_) => Ok(Json(ApiResponse::new("User created successfully".to_string()))),
        Err(err) => Err(ApplicationErrors::Database(err.to_string())),
    }
}

pub async fn update_user(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>, 
    Form(user_details): Form<User>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    
    match app_state.database.update_user(id, user_details).await {
        Ok(_) => Ok(Json(ApiResponse::new("User updated successfully".to_string()))),
        Err(err) => Err(ApplicationErrors::Database(err.to_string())),
    }
}

pub async fn get_all_users(
    State(app_state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>
) -> Result<Json<ApiResponse<Vec<User>>>, ApplicationErrors> {
    
    let limit = pagination.limit.unwrap_or(20);
    
    match app_state.database.get_all_users(pagination.last_id, limit).await {
        Ok(users) => Ok(Json(ApiResponse::new(users))),
        Err(err) => Err(ApplicationErrors::Database(err.to_string())),
    }
}