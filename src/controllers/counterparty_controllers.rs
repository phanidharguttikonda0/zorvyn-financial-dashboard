use std::sync::Arc;
use axum::extract::{Path, State, Form};
use crate::AppState;
use crate::models::counterparty_models::Party;

pub async fn create_party(State(app_state):State<Arc<AppState>>, Form(party): Form<Party>) {}

pub async fn delete_party(State(app_state):State<Arc<AppState>>, Path(id):Path<i64>) {}

pub async fn update_party(State(app_state):State<Arc<AppState>>, Path(id):Path<i64>, Form(party): Form<Party>) {}

pub async fn get_parties(State(app_state):State<Arc<AppState>>) {}