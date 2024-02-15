use std::sync::Arc;

use axum::{http::StatusCode, routing, Json, Router};

use crate::state::AppState;

mod auth;
mod openai;

pub fn routes(state: Arc<AppState>) -> Router<()> {
    let openai_routes = openai::routes(state.clone());
    Router::new()
        .route(
            "/health",
            routing::get(|| async {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({"status": "healthy"})),
                )
            }),
        )
        .with_state(state)
        .nest("/ai", openai_routes)
}
