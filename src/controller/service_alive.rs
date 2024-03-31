use crate::utils::{build_response, Response};
use axum::{http::StatusCode, Json};

pub async fn alive_route() -> (StatusCode, Json<Response>) {
    build_response(StatusCode::OK, "O gateway está online!!".to_string())
}
