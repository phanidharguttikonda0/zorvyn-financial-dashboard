use std::sync::Arc;
use axum::{Form, Json};
use axum::extract::{Path, State};
use validator::Validate;
use tracing::{info, error};

use crate::AppState;
use crate::models::counterparty_models::Party;
use crate::services::errors::{ApiResponse, ApplicationErrors};

pub async fn create_party(
    State(app_state): State<Arc<AppState>>,
    Form(party): Form<Party>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Creating a new counterparty: {:?}", party.name);
    
    if let Err(e) = party.validate() {
        error!("Validation failed for counterparty: {}", e);
        return Err(ApplicationErrors::Validation(e.to_string()));
    }

    match app_state.database.create_party(party).await {
        Ok(_) => {
            info!("Counterparty created successfully.");
            Ok(Json(ApiResponse::new("Counterparty created successfully".to_string())))
        },
        Err(e) => {
            error!("Failed to create counterparty: {}", e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn get_parties(
    State(app_state): State<Arc<AppState>>
) -> Result<Json<ApiResponse<Vec<Party>>>, ApplicationErrors> {
    info!("Fetching all counterparties");

    match app_state.database.get_parties().await {
        Ok(parties) => Ok(Json(ApiResponse::new(parties))),
        Err(e) => {
            error!("Failed to fetch counterparties: {}", e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn get_party(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<Party>>, ApplicationErrors> {
    info!("Fetching counterparty with id bounds: {}", id);

    match app_state.database.get_party(id).await {
        Ok(party) => Ok(Json(ApiResponse::new(party))),
        Err(e) => {
            error!("Failed to fetch counterparty {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn update_party(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>, 
    Form(party): Form<Party>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Updating counterparty: {}", id);
    if let Err(e) = party.validate() {
        error!("Validation failed for counterparty update: {}", e);
        return Err(ApplicationErrors::Validation(e.to_string()));
    }

    match app_state.database.update_party(id, party).await {
        Ok(_) => Ok(Json(ApiResponse::new("Counterparty updated successfully".to_string()))),
        Err(e) => {
            error!("Failed to update counterparty {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}

pub async fn delete_party(
    State(app_state): State<Arc<AppState>>, 
    Path(id): Path<i64>
) -> Result<Json<ApiResponse<String>>, ApplicationErrors> {
    info!("Deleting counterparty: {}", id);

    match app_state.database.delete_party(id).await {
        Ok(_) => {
            info!("Counterparty {} deleted.", id);
            Ok(Json(ApiResponse::new("Counterparty deleted successfully".to_string())))
        },
        Err(e) => {
            error!("Failed to delete counterparty {}: {}", id, e);
            Err(ApplicationErrors::Database(e.to_string()))
        }
    }
}