pub mod auth0;
pub mod authenticator;
mod errors;
mod noop;
mod openid;

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

use crate::{app::auth::errors::AuthError, state::AppState};

pub async fn simple_route_guard(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    let auth_header = headers.get("Authorization");
    match auth_header {
        Some(opaque) => match opaque.to_str() {
            Ok(bearer_token) => {
                log::info!("{bearer_token}");
                let authenticator = &state.services.auth;
                let token: &str = match bearer_token.len() > 7 {
                    true => &bearer_token[7..],
                    _ => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(serde_json::json!({"msg": "malformed auth header"})),
                        )
                            .into_response()
                    }
                };
                let user_data = authenticator.authenticate(token).await;
                match user_data {
                    Ok(_) => {
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
            Err(_) => (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"msg": "missing authorization header"})),
            )
                .into_response(),
        },
        None => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"msg": "missing authorization header"})),
        )
            .into_response(),
    }
}
