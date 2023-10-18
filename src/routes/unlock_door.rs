use crate::mqtt_actions::MQTT_UNLOCK_TOPIC;
use axum::{http::StatusCode, Json};
use rumqttc::AsyncClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Password {
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn unlock_door(
    Json(body): Json<Password>,
    mqtt_cli: Arc<AsyncClient>,
) -> (StatusCode, Json<Response>) {
    if body.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Response {
                message: "Invalid Body".to_string(),
            }),
        );
    }

    // TODO: Connect on a database to get password hash
    // TODO: Add password validation logic

    // TODO: Send mqtt message to a topic to unlock the door
    mqtt_cli
        .publish(MQTT_UNLOCK_TOPIC, rumqttc::QoS::AtLeastOnce, true, "1")
        .await
        .unwrap();

    return (
        StatusCode::OK,
        Json(Response {
            message: "Password is valid".to_string(),
        }),
    );
}
