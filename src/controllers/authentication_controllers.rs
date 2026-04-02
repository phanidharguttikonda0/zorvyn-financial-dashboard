use std::sync::Arc;
use axum::extract::{State, Form};
use axum::Json;
use axum::response::IntoResponse;
use tracing::info;
use validator::Validate;
use crate::AppState;
use crate::models::authentication_models::{AuthorizationToken, Claims, SignIn, UserInfo};
use crate::services::errors::{ApiResponse, ApplicationErrors};
use base64 ;
use crate::services::authentication_services::{generate_jwt_token, verify_password};

pub async fn sign_in(State(app_state): State<Arc<AppState>>, Form(sign_in):Form<SignIn>)
                     -> Result<Json<ApiResponse<AuthorizationToken>>, ApplicationErrors> {

    match sign_in.validate() {
        Ok(_) => {
            info!("Sign-in validated");
        },
        Err(err) => {
            return Err(ApplicationErrors::Validation(err.to_string()));
        }
    } ;

    info!("getting userinfo from the database") ;

    let (actual_password, user_info, status) = match app_state.database.get_user_password(sign_in.email.as_str()).await {
        Ok(p) => p,
        Err(err) => {
             return Err(ApplicationErrors::Database(err.to_string()))
        }
    } ;
    info!("checking status of the account") ;
    if status.eq("inactive") {
        return Err(ApplicationErrors::Database("The Account was Inactive".to_string()))
    }
    
    info!("verifying password") ;
    if verify_password(sign_in.password.as_str(), &actual_password) {
        Ok(Json(ApiResponse::new(AuthorizationToken {
            access_token: generate_jwt_token(user_info),
            token_type: "Bearer".to_string(),
        })))
    }else {
        Err(ApplicationErrors::Validation("Invalid password.".to_string()))
    }
}



