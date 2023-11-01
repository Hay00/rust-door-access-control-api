use axum::{routing::get, routing::post, Router};

use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub mod mqtt_actions;
pub mod mysql_actions;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Error)
        .init();

    // Initialize MQTT client
    mqtt_actions::init_main_client();
    let route_cli = Arc::new(mqtt_actions::init_route_client());

    // Initialize SQL connection
    let db_conn = Arc::new(Mutex::new(
        mysql_actions::init_db_connection().await.unwrap(),
    ));

    let validate_params = {
        let shared_mqtt_cli = Arc::clone(&route_cli);
        let shared_sql_conn = Arc::clone(&db_conn);
        move |json| routes::unlock_door(json, shared_mqtt_cli, shared_sql_conn)
    };

    let app = Router::new()
        .route("/", get(routes::alive_route))
        .route("/validate-password", post(validate_params));

    let host = env::var("GCA_ACCESS_SERVER_HOST").expect("GCA_ACCESS_SERVER_HOST not set");
    let port = env::var("GCA_ACCESS_SERVER_PORT").expect("GCA_ACCESS_SERVER_PORT not set");

    let addr = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Invalid address or port");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
