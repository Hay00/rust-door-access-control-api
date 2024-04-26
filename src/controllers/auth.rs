use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::models::user;
use crate::utils::errors::ControllerError;
use crate::utils::MappedErrors;
use crate::{auth, AppState};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[debug_handler]
pub async fn login(
    State(app_state): State<AppState>,
    login_data: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ControllerError> {
    let user = user::find_by_login_user(
        &app_state.db_pool,
        login_data.email.clone(),
        login_data.password.clone(),
    )
    .await
    .map_err(|err| match err {
        MappedErrors::NotFound => ControllerError {
            message: "Invalid user".to_string(),
            status_code: StatusCode::NOT_FOUND,
        },
        MappedErrors::InternalServerError => ControllerError {
            message: "Internal server error".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        },
    })?;

    let token = auth::generate_token(user.id, user.email);

    match token {
        Ok(token) => Ok(Json(LoginResponse { token })),
        Err(err) => Err(ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}
