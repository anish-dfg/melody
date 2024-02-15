use std::sync::Arc;

use axum::{middleware, routing, Router};

use crate::{app::auth, state::AppState};

use self::controllers::post_chat_message;

mod controllers;

pub fn routes(state: Arc<AppState>) -> Router<()> {
    Router::new()
        // .route("/", routing::post(post_chat_message))
        .route("/completions", routing::get(|| async move { "here" }))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth::route_guard,
        ))
        .with_state(state)
}