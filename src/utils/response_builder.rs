use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub fn build_response(status: StatusCode, message: String) -> (StatusCode, Json<Response>) {
    (status, Json(Response { message }))
}
