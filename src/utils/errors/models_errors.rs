use axum::{http::StatusCode, response::IntoResponse};

pub enum ModelError {
    NotFound,
    InternalServerError,
}

impl IntoResponse for ModelError {
    fn into_response(self) -> axum::response::Response {
        let message = match self {
            ModelError::NotFound => "Model not found",
            ModelError::InternalServerError => "Internal server error",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}
