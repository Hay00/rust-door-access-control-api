use log::{error, info};
use rumqttc::{AsyncClient, EventLoop, LastWill, MqttOptions, QoS};
use std::{thread, time::Duration};
use tokio::task;

use crate::services::mqtt::{publish_online_status, MQTT_STATUS_TOPIC, OFFLINE_STATUS};

async fn connection_poll(mut con: EventLoop) {
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

fn setup_mqtt_options(id: String) -> MqttOptions {
    let mqtt_host = std::env::var("GCA_DOOR_MQTT_HOST").expect("GCA_MQTT_HOST not set");
    let mqtt_port = std::env::var("GCA_DOOR_MQTT_PORT").expect("GCA_MQTT_PORT not set");
    let mqtt_user = std::env::var("GCA_DOOR_MQTT_USER").expect("GCA_MQTT_USER not set");
    let mqtt_pass = std::env::var("GCA_DOOR_MQTT_PASS").expect("GCA_MQTT_PASS not set");

    let mut options = MqttOptions::new(id, mqtt_host, mqtt_port.parse::<u16>().unwrap());
    options
        .set_credentials(mqtt_user, mqtt_pass)
        .set_clean_session(true)
        .set_keep_alive(Duration::from_secs(5));

    options
}

pub fn init_main_client() {
    let mut mqtt_options = setup_mqtt_options("api-gateway-will".to_string());
    let will = LastWill::new(MQTT_STATUS_TOPIC, OFFLINE_STATUS, QoS::AtLeastOnce, true);
    mqtt_options.set_last_will(will);

    let (main_cli, main_loop) = AsyncClient::new(mqtt_options, 10);

    task::spawn(connection_poll(main_loop));
    task::spawn(async move {
        publish_online_status(&main_cli).await;
    });
}

pub fn init_route_client() -> AsyncClient {
    let mqtt_options_route = setup_mqtt_options("api-gateway-main".to_string());
    let (route_cli, route_loop) = AsyncClient::new(mqtt_options_route, 10);
    task::spawn(connection_poll(route_loop));

    route_cli
}
