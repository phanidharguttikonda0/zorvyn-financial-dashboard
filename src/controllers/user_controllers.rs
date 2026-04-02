use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Form;
use crate::AppState;
use crate::models::user_models::User;

pub async fn create_user(State(app_state):State<Arc<AppState>>, Form(user_details):Form<User>) {
    // here remaining will be none expect name, email, password and role. status will be active by default
}

pub async fn update_user(State(app_state):State<Arc<AppState>>, Path(id):Path<i64>, Form(user_details):Form<User>) {
    // here which ever user_details are not none, we are going to use those
}

pub async fn get_all_users(State(app_state):State<Arc<AppState>>) {
    // we need to implement cursor pagination, we need to get last user and limit to get those list of users
    
}