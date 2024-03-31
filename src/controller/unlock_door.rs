use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use validator::Validate;

use crate::models::{User, UserAccess};
use crate::services::mqtt;
use crate::utils::{build_response, Response};
use crate::AppState;

#[derive(Deserialize, Validate, Debug)]
pub struct UserAuth {
    #[validate(length(min = 3))]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

pub async fn unlock_door(
    State(state): State<AppState>,
    Json(body): Json<UserAuth>,
) -> (StatusCode, Json<Response>) {
    // Validate body fields
    if body.validate().is_err() {
        return build_response(StatusCode::BAD_REQUEST, "Invalid request".to_string());
    }

    // Find user by email and password, if not found return unauthorized
    let user_search = User::find(&state.db_pool, body.email.clone(), body.password.clone()).await;
    let user_id = match user_search {
        Err(_) => return build_response(StatusCode::UNAUTHORIZED, "Invalid User".to_string()),
        Ok(id) => id,
    };

    // Validate if user has access on the current time, if not return unauthorized
    let access_search = UserAccess::has_access_now(&state.db_pool, user_id).await;
    let _is_user_valid = match access_search {
        Err(_) => {
            return build_response(StatusCode::UNAUTHORIZED, "User has no access".to_string())
        }
        Ok(access) => access,
    };

    // Unlock door using MQTT
    mqtt::publish_open_door(&state.mqtt_cli).await;

    build_response(StatusCode::OK, "Valid user".to_string())
}
