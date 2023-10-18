use axum::{routing::get, routing::post, Router};
use std::{env, net::SocketAddr, sync::Arc};

pub mod mqtt_actions;
pub mod routes;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    mqtt_actions::init_main_client();
    let route_cli = Arc::new(mqtt_actions::init_route_client());

    let auth = {
        let shared_cli = Arc::clone(&route_cli);
        move |path| routes::unlock_door(path, shared_cli)
    };

    let app = Router::new()
        .route("/", get(routes::alive_route))
        .route("/validate-password", post(auth));

    let (host, port) = from_env();
    let addr = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Invalid address or port");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn from_env() -> (String, String) {
    (
        env::var("GCA_ACCESS_SERVER_HOST")
            .ok()
            .unwrap_or_else(|| "127.0.0.1".to_string()),
        env::var("GCA_ACCESS_SERVER_PORT")
            .ok()
            .unwrap_or_else(|| "8080".to_string()),
    )
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
