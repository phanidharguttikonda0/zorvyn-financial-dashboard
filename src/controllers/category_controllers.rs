use std::sync::Arc;
use axum::{Form, Json};
use axum::extract::{Path, State};
use validator::Validate;
use tracing::{info, error};

use crate::AppState;
use crate::models::category_models::Category;
use crate::services::errors::{ApiResponse, ApplicationErrors};

pub async fn create_category(
    State(app_state): State<Arc<AppState>>,
    Form(category): Form<Category>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Creating a new category: {:?}", category.name);
    
    if let Err(e) = category.validate() {
        error!("Validation failed for category: {}", e);
        return Err(ApplicationErrors::Validation(e.to_string()));
    }

    match app_state.database.create_category(category).await {
        Ok(_) => {
            info!("Category created successfully.");
            Ok(Json(ApiResponse::new("Category created successfully".to_string())))
        },
        Err(e) => {
            error!("Failed to create category: {}", e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn get_categories(
    State(app_state): State<Arc<AppState>>
) -> Result<Json<ApiResponse<Vec<Category>>>, ApplicationErrors> {
    info!("Fetching all categories");

    match app_state.database.get_categories().await {
        Ok(categories) => Ok(Json(ApiResponse::new(categories))),
        Err(e) => {
            error!("Failed to fetch categories: {}", e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn get_category(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<Category>>, ApplicationErrors> {
    info!("Fetching category with id bounds: {}", id);

    match app_state.database.get_category(id).await {
        Ok(category) => Ok(Json(ApiResponse::new(category))),
        Err(e) => {
            error!("Failed to fetch category {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn update_category(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>, 
    Form(category): Form<Category>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Updating category: {}", id);
    if let Err(e) = category.validate() {
        error!("Validation failed for category update: {}", e);
        return Err(ApplicationErrors::Validation(e.to_string()));
    }

    match app_state.database.update_category(id, category).await {
        Ok(_) => Ok(Json(ApiResponse::new("Category updated successfully".to_string()))),
        Err(e) => {
            error!("Failed to update category {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn delete_category(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Deleting category: {}", id);

    match app_state.database.delete_category(id).await {
        Ok(_) => {
            info!("Category {} deleted.", id);
            Ok(Json(ApiResponse::new("Category deleted successfully".to_string())))
        },
        Err(e) => {
            error!("Failed to delete category {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}