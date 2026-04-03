use std::sync::Arc;
use axum::{Form, Json};
use axum::extract::{State, Path};
use validator::Validate;
use tracing::{info, error};

use crate::AppState;
use crate::models::transaction_models::Transaction;
use crate::services::errors::{ApiResponse, ApplicationErrors};

pub async fn create_transaction(
    State(app_state): State<Arc<AppState>>, 
    Form(transaction): Form<Transaction>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Creating a new transaction");
    
    if let Err(e) = transaction.validate() {
        error!("Validation failed for transaction: {}", e);
        return Err(ApplicationErrors::Validation(e.to_string()));
    }

    match app_state.database.create_transaction(transaction).await {
        Ok(_) => {
            info!("Transaction created successfully.");
            Ok(Json(ApiResponse::new("Transaction created successfully".to_string())))
        },
        Err(e) => {
            error!("Failed to create transaction: {}", e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn delete_transaction(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Deleting transaction: {}", id);

    match app_state.database.delete_transaction(id).await {
        Ok(_) => {
            info!("Transaction {} deleted.", id);
            Ok(Json(ApiResponse::new("Transaction deleted successfully".to_string())))
        },
        Err(e) => {
            error!("Failed to delete transaction {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn update_transaction(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>, 
    Form(transaction): Form<Transaction>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Updating transaction: {}", id);
    if let Err(e) = transaction.validate() {
        error!("Validation failed for transaction update: {}", e);
        return Err(ApplicationErrors::Validation(e.to_string()));
    }

    match app_state.database.update_transaction(id, transaction).await {
        Ok(_) => Ok(Json(ApiResponse::new("Transaction updated successfully".to_string()))),
        Err(e) => {
            error!("Failed to update transaction {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn get_transactions(
    State(app_state): State<Arc<AppState>>
) -> Result<Json<ApiResponse<Vec<Transaction>>>, ApplicationErrors> {
    info!("Fetching all transactions");

    // we need to add cursor pagination along with some filters need to figure it out
    match app_state.database.get_transactions().await {
        Ok(transactions) => Ok(Json(ApiResponse::new(transactions))),
        Err(e) => {
            error!("Failed to fetch transactions: {}", e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn get_transaction(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<Transaction>>, ApplicationErrors> {
    info!("Fetching transaction with id bounds: {}", id);

    match app_state.database.get_transaction(id).await {
        Ok(transaction) => Ok(Json(ApiResponse::new(transaction))),
        Err(e) => {
            error!("Failed to fetch transaction {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}