use axum::extract::{self, FromRequest, Request, State};
use axum::{async_trait, body::Bytes, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{user, users_accesses};
use crate::services::mqtt;
use crate::utils::{
    errors::{ControllerError, ControllerErrorType},
    Response,
};
use crate::AppState;

#[derive(Serialize, Debug, Deserialize, Validate)]
pub struct UserAuth {
    #[validate(email(message = "Email inválido"))]
    email: String,

    #[validate(length(min = 8, message = "Senha deve ter no mínimo 8 caracteres"))]
    password: String,
}

// Make impl for FromRequest to validate body fields
#[async_trait]
impl<S> FromRequest<S> for UserAuth
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = ControllerError;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let user = extract::Json::<UserAuth>::from_request(request, state)
            .await
            .map(|Json(user)| user)
            .map_err(|error| ControllerError::validation_error(error.to_string()))?;

        // Validate body
        if let Err(_) = user.validate() {
            return Err(ControllerError::from_type(
                ControllerErrorType::BodyParsingError,
            ));
        }

        Ok(user)
    }
}

#[debug_handler]
pub async fn unlock(
    State(state): State<AppState>,
    user: UserAuth,
) -> Result<Json<Response>, ControllerError> {
    // Find user by email and password, if not found return unauthorized
    let user_search =
        user::find_by_login(&state.db_pool, user.email.clone(), user.password.clone()).await;
    let user_id = match user_search {
        Err(_) => {
            return Err(ControllerError {
                message: "Usuário inválido".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
        Ok(id) => id,
    };

    // Validate if user has access on the current time, if not return unauthorized
    let access_search = users_accesses::has_access_now(&state.db_pool, user_id).await;
    let _is_user_valid = match access_search {
        Err(_) => {
            return Err(ControllerError {
                message: "Usuário não tem acesso no momento".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
        Ok(access) => access,
    };

    // Publish open door message
    mqtt::publish_open_door(&state.mqtt_cli).await;

    Ok(Json(Response {
        message: "Porta destrancada".to_string(),
    }))
}
