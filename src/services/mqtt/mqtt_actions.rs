use rumqttc::{AsyncClient, QoS};

use crate::services::mqtt::{MQTT_STATUS_TOPIC, MQTT_UNLOCK_TOPIC, ONLINE_STATUS};

pub async fn publish_online_status(cli: &AsyncClient) {
    cli.publish(
        MQTT_STATUS_TOPIC,
        QoS::AtLeastOnce,
        true,
        ONLINE_STATUS.as_bytes(),
    )
    .await
    .unwrap();
}

pub async fn publish_open_door(cli: &AsyncClient) {
    cli.publish(
        MQTT_UNLOCK_TOPIC,
        rumqttc::QoS::AtLeastOnce,
        false,
        "true".as_bytes(),
    )
    .await
    .unwrap();
}
