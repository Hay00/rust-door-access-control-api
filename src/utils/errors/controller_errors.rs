use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use core::fmt;

use serde_json::json;

#[derive(Debug)]
pub struct ControllerError {
    pub message: String,
    pub status_code: StatusCode,
}

impl ControllerError {
    pub fn new(message: String, status_code: StatusCode) -> Self {
        Self {
            message,
            status_code,
        }
    }

    pub fn validation_error(message: String) -> Self {
        Self {
            message,
            status_code: StatusCode::BAD_REQUEST,
        }
    }

    pub fn from_type(error_type: ControllerErrorType) -> Self {
        let (message, status_code) = match error_type {
            ControllerErrorType::BodyParsingError => {
                ("Request body inválido".to_string(), StatusCode::BAD_REQUEST)
            }
            ControllerErrorType::NotFound => {
                ("Recurso não encontrado".to_string(), StatusCode::NOT_FOUND)
            }
            ControllerErrorType::Unauthorized => {
                ("Não autorizado".to_string(), StatusCode::UNAUTHORIZED)
            }
            ControllerErrorType::InternalServerError => (
                "Erro interno".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        Self {
            message,
            status_code,
        }
    }
}

pub enum ControllerErrorType {
    BodyParsingError,
    NotFound,
    Unauthorized,
    InternalServerError,
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code;
        let header = [(header::CONTENT_TYPE, "application/json")];
        let body = Json(json!({
            "status_code": self.status_code.as_u16(),
            "message": self.message,
        }));

        (status_code, header, body).into_response()
    }
}

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
