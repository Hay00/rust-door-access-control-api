use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::models::user;
use crate::utils::errors::ControllerError;
use crate::utils::MappedErrors;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct UsersListResponse {
    users: Vec<user::ListUser>,
}

#[debug_handler]
pub async fn create_user(
    State(app_state): State<AppState>,
    user_data: Json<user::CreateUser>,
) -> Result<StatusCode, ControllerError> {
    user::create(&app_state.db_pool, user_data.0)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::CREATED)
}

#[debug_handler]
pub async fn find_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<u32>,
) -> Result<Json<user::ListUser>, ControllerError> {
    let user = user::find(&app_state.db_pool, user_id)
        .await
        .map_err(|err| {
            let status_code = match err {
                MappedErrors::NotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            ControllerError {
                message: err.to_string(),
                status_code: status_code,
            }
        })?;

    Ok(Json(user))
}

// TODO: Implement pagination
#[debug_handler]
pub async fn list_all(
    State(app_state): State<AppState>,
) -> Result<Json<UsersListResponse>, ControllerError> {
    let users = user::list(&app_state.db_pool)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(UsersListResponse { users }))
}

#[debug_handler]
pub async fn update_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<u32>,
    user_data: Json<user::UpdateUser>,
) -> Result<StatusCode, ControllerError> {
    user::update(&app_state.db_pool, user_id, user_data.0)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn delete_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<u32>,
) -> Result<StatusCode, ControllerError> {
    user::disable(&app_state.db_pool, user_id)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::OK)
}
