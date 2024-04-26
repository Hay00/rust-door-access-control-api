use axum_macros::FromRef;
use deadpool_diesel::mysql::Pool;
use std::{env, net::SocketAddr, sync::Arc};

pub mod auth;
pub mod controllers;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Clone, FromRef)]
pub struct AppState {
    db_pool: Pool,
    mqtt_cli: Arc<rumqttc::AsyncClient>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Error)
        .init();

    // Initialize MQTT client
    services::mqtt::init_main_client();
    let route_cli = Arc::new(services::mqtt::init_route_client().to_owned());

    // Initialize AppState, shared state between routes
    let state = AppState {
        db_pool: services::sql::establish_connection(),
        mqtt_cli: route_cli,
    };

    let app = routes::builder(state);

    let host = env::var("GCA_ACCESS_SERVER_HOST").expect("GCA_ACCESS_SERVER_HOST not set");
    let port = env::var("GCA_ACCESS_SERVER_PORT").expect("GCA_ACCESS_SERVER_PORT not set");

    let addr = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Invalid address or port");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("\nSignal shutdown, exiting application...");
}
