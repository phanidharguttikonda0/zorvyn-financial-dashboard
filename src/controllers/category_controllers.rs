use std::sync::Arc;
use axum::extract::{State, Path, Form};
use axum::response::IntoResponse;
use crate::AppState;
use crate::models::category_models::Category;

pub async fn create_category(State(app_state):State<Arc<AppState>>, Form(category): Form<Category>) -> impl IntoResponse {}

pub async fn delete_category(State(app_state):State<Arc<AppState>>, Path(id): Path<i64>) {}

pub async fn update_category(State(app_state):State<Arc<AppState>>, Path(id): Path<i64>, Form(category): Form<Category>) -> impl IntoResponse {}

pub async fn get_categories(State(app_state):State<Arc<AppState>>) {
    // we need to implement cursor pagination
}

pub async fn get_category(State(app_state):State<Arc<AppState>>, Path(id): Path<i64>) {}