use std::sync::Arc;
use axum::extract::State;
use crate::AppState;

pub async fn get_summary(State(app_state):State<Arc<AppState>>) {}

pub async fn get_recent(State(app_state):State<Arc<AppState>>) {}

pub async fn get_by_category(State(app_state):State<Arc<AppState>>) {}

pub async fn get_by_trends(State(app_state):State<Arc<AppState>>) {}