pub mod auth0;
pub mod authenticator;
mod errors;
pub mod noop;

#[cfg(test)]
mod tests;

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use crate::state::AppState;

pub async fn simple_route_guard(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    let auth_header = headers.get("Authorization");

    let raw_auth_header = match auth_header {
        Some(opaque) => match opaque.to_str() {
            Ok(bearer_token) => bearer_token,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"msg": "missing authorization header"})),
                )
                    .into_response()
            }
        },
        None => "",
    };

    let token: &str = if raw_auth_header.len() > 7 {
        &raw_auth_header[7..]
    } else {
        raw_auth_header
    };

    let authenticator = &state.services.auth;
    match authenticator.authenticate(token).await {
        Ok(data) => {
            let res = next.run(req).await;
            res
        }
        Err(e) => {
            log::error!("{}", e.to_string());
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"msg": "unauthorized"})),
            )
                .into_response()
        }
    }
}
