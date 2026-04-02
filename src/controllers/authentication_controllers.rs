use std::sync::Arc;
use axum::extract::{State, Form};
use crate::AppState;
use crate::models::authentication_models::SignIn;

pub async fn sign_in(State(app_state): State<Arc<AppState>>, Form(sign_in):Form<SignIn>) {}