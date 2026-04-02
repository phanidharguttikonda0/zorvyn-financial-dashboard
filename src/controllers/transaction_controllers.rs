
use std::sync::Arc;
use axum::extract::{State, Form, Path};
use crate::AppState;
use crate::models::transaction_models::Transaction;

pub async fn create_transaction(State(app_state): State<Arc<AppState>>, Form(transaction): Form<Transaction>) {
    
}

pub async fn delete_transaction(State(app_state): State<Arc<AppState>>, Path(transaction_id): Path<i64>) {

}

pub async fn update_transaction(State(app_state): State<Arc<AppState>>, Path(id): Path<i64>, Form(transaction): Form<Transaction>) {

}

pub async fn get_transactions(State(app_state): State<Arc<AppState>>) {
    // we need to add cursor pagination along with some filters need to figure it out
}

pub async fn get_transaction(State(app_state): State<Arc<AppState>>, Path(id): Path<i64>) {}