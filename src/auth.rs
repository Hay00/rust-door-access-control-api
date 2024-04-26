use axum::http::StatusCode;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use log::error;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{Duration, SystemTime};

use crate::utils::errors::ControllerError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub email: String,
    pub iat: u64,
    pub exp: u64,
}

fn fetch_secret() -> String {
    env::var("GCA_SECRET_KEY").expect("DATABASE_URL must be set")
}

pub fn generate_token(user_id: i32, email: String) -> Result<String, ControllerError> {
    let secret = fetch_secret();
    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            error!("Unable to get current time, system time is before UNIX EPOCH");
            return Err(ControllerError {
                message: "Error generating token".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            });
        }
    };

    let one_day = 60 * 60 * 24;
    let claims = Claims {
        user_id,
        email,
        iat: current_time,
        exp: current_time + Duration::from_secs(one_day).as_secs(),
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => {
            error!("Error during token generation, check the secret key");
            return Err(ControllerError {
                message: "Error generating token".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            });
        }
    }
}

pub fn validate_token(token: String) -> bool {
    let secret = fetch_secret();
    let validation = jsonwebtoken::Validation::default();

    match jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(_) => true,
        Err(_) => false,
    }
}
