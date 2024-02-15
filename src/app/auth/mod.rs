pub mod auth0;
pub mod authenticator;
mod errors;
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

use crate::state::AppState;

pub async fn route_guard(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Response {
    let res = next.run(req).await;
    let auth_header = headers.get("Authorization");
    match auth_header {
        Some(opaque) => match opaque.to_str() {
            Ok(bearer_token) => {
                log::info!("{bearer_token}");
                res
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
