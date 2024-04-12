use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn intercept_request(request: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match is_token_valid(auth_header) {
        true => Ok(next.run(request).await),
        false => Err(StatusCode::UNAUTHORIZED),
    }
}

fn is_token_valid(token: Option<&str>) -> bool {
    token.is_some()
}
