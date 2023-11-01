use crate::{mqtt_actions::MQTT_UNLOCK_TOPIC, mysql_actions};
use axum::{http::StatusCode, Json};
use rumqttc::AsyncClient;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct UserAuth {
    #[validate(length(min = 3))]
    user: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn unlock_door(
    Json(body): Json<UserAuth>,
    mqtt_cli: Arc<AsyncClient>,
    sql_conn: Arc<Mutex<mysql::PooledConn>>,
) -> (StatusCode, Json<Response>) {
    // Validate body
    if body.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(Response {
                message: "Invalid Body".to_string(),
            }),
        );
    }

    // Validate user
    let is_user_valid = mysql_actions::is_user_valid(sql_conn, body.user, body.password).await;
    if !is_user_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(Response {
                message: "Invalid User".to_string(),
            }),
        );
    }

    // Unlock door using MQTT
    mqtt_cli
        .publish(
            MQTT_UNLOCK_TOPIC,
            rumqttc::QoS::AtLeastOnce,
            false,
            "true".as_bytes(),
        )
        .await
        .unwrap();

    return (
        StatusCode::OK,
        Json(Response {
            message: "Valid user".to_string(),
        }),
    );
}
