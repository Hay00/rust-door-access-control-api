use log::{error, info};
use rumqttc::{AsyncClient, EventLoop, LastWill, MqttOptions, QoS};
use std::{thread, time::Duration};
use tokio::task;

const ONLINE_STATUS: &'static str = "online";
const OFFLINE_STATUS: &'static str = "offline";
const MQTT_STATUS_TOPIC: &'static str = "gca/api-gateway/status";
pub const MQTT_UNLOCK_TOPIC: &'static str = "gca/api-gateway/unlock";

pub async fn connection_poll(mut con: EventLoop) {
    loop {
        let evt = con.poll().await;
        match evt {
            Ok(msg) => {
                info!("Received notification: {:?}", msg);
            }
            Err(e) => {
                error!("Error receiving notification: {:?}", e);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}

async fn publish_online_status(cli: &AsyncClient) {
    cli.publish(
        MQTT_STATUS_TOPIC,
        QoS::AtLeastOnce,
        true,
        ONLINE_STATUS.as_bytes(),
    )
    .await
    .unwrap();
}

pub fn init_main_client() {
    let mut mqtt_options = MqttOptions::new("api-gateway", "127.0.0.1", 1883);
    let will = LastWill::new(MQTT_STATUS_TOPIC, OFFLINE_STATUS, QoS::AtLeastOnce, true);
    mqtt_options
        .set_keep_alive(Duration::from_secs(5))
        .set_last_will(will);

    let (main_cli, main_loop) = AsyncClient::new(mqtt_options, 10);

    task::spawn(connection_poll(main_loop));
    task::spawn(async move {
        publish_online_status(&main_cli).await;
    });
}

pub fn init_route_client() -> AsyncClient {
    let mut mqtt_options_route = MqttOptions::new("api-gatewayy", "127.0.0.1", 1883);
    mqtt_options_route.set_keep_alive(Duration::from_secs(5));

    let (route_cli, route_loop) = AsyncClient::new(mqtt_options_route, 10);
    task::spawn(connection_poll(route_loop));

    route_cli
}
