use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{app::openai, state::AppState};

pub async fn post_chat_message(
    State(state): State<Arc<AppState>>,
    Json(data): Json<openai::chat::ChatOptions>,
) -> (StatusCode, Response) {
    match state.services.ai.get_chat_completion(&data).await {
        Ok(chat) => (StatusCode::OK, Json(chat).into_response()),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"msg": "unable to retrieve chat completion"})).into_response(),
        ),
    }
}
