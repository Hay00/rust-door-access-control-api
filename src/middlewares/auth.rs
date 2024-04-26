use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use log::warn;
use serde::{Deserialize, Serialize};

use crate::auth::validate_token;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}

pub async fn intercept_request(request: Request, next: Next) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            header
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });

    let token = match token {
        Some(token) => token,
        None => {
            warn!("No token provided");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    match validate_token(token) {
        true => Ok(next.run(request).await),
        false => Err(StatusCode::UNAUTHORIZED),
    }
}
