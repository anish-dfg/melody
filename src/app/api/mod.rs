pub mod v1;

use std::sync::Arc;

use axum::{routing, Router};

use crate::state::AppState;

pub fn routes(state: Arc<AppState>) -> Router<()> {
    let v1_routes = v1::routes(state);
    Router::new().nest("/v1", v1_routes)
}
