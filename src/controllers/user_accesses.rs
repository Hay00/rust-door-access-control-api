use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum_macros::debug_handler;
use serde::Serialize;

use crate::models::users_accesses::{self, UserAccess};
use crate::utils::errors::ControllerError;
use crate::AppState;

// Users list response type
#[derive(Debug, Serialize)]
pub struct AccessesListResponse {
    accesses: Vec<users_accesses::UserAccess>,
}

#[debug_handler]
pub async fn create_access(
    State(app_state): State<AppState>,
    Path(user_id): Path<u32>,
    access_data: Json<users_accesses::UserAccessCreate>,
) -> Result<StatusCode, ControllerError> {
    let access = UserAccess {
        user_id: user_id as i32,
        day_of_week: access_data.day_of_week,
        start: access_data.start,
        end: access_data.end,
    };

    users_accesses::create(&app_state.db_pool, access)
        .await
        .map_err(|err| ControllerError {
            // TODO: Parse error response for duplicity
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::CREATED)
}

#[debug_handler]
pub async fn find_by_user(
    State(app_state): State<AppState>,
    Path(user_id): Path<u32>,
) -> Result<Json<AccessesListResponse>, ControllerError> {
    let accesses = users_accesses::find(&app_state.db_pool, user_id)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(AccessesListResponse { accesses }))
}

// TODO: Implement pagination
#[debug_handler]
pub async fn list_all(
    State(app_state): State<AppState>,
) -> Result<Json<AccessesListResponse>, ControllerError> {
    let accesses = users_accesses::list_all(&app_state.db_pool)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(AccessesListResponse { accesses }))
}

#[debug_handler]
pub async fn update_access(
    State(app_state): State<AppState>,
    Path((user_id, day_id)): Path<(u32, u32)>,
    access_data: Json<users_accesses::UserAccessUpdate>,
) -> Result<StatusCode, ControllerError> {
    users_accesses::update(&app_state.db_pool, user_id, day_id, access_data.0)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn delete_access(
    State(app_state): State<AppState>,
    Path((user_id, day_id)): Path<(u32, u32)>,
) -> Result<StatusCode, ControllerError> {
    users_accesses::delete(&app_state.db_pool, user_id, day_id)
        .await
        .map_err(|err| ControllerError {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(StatusCode::OK)
}
