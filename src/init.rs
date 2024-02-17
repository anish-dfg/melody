use axum::{http::StatusCode, Json, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    app::{
        api,
        auth::{auth0::Auth0, authenticator::Authenticator, noop::NoOpAuth},
        openai::OpenAIClient,
        storage::{cache, sql},
        types::AssetBackend,
    },
    launch::LaunchMode,
    state::{AppState, Config, ServiceLayer, StorageLayer},
};
use std::{env, sync::Arc};

/// Create app configuration
fn build_config() -> Config {
    let name = env::var("APP_NAME").expect("missing app name");
    let app_secret = env::var("APP_SECRET")
        .map(|s| s.as_bytes().to_vec())
        .expect("missing app secret");

    let launch_mode = match env::var("LAUNCH_MODE")
        .expect("missing launch mode")
        .as_str()
    {
        "development" => LaunchMode::Development,
        "testing" => LaunchMode::Testing,
        "staging" => LaunchMode::Staging,
        _ => LaunchMode::Production,
    };
    let asset_backend = match env::var("ASSET_BACKEND")
        .expect("missing asset backend")
        .to_lowercase()
        .as_str()
    {
        "aws" | "amazon" | "s3" => AssetBackend::Aws,
        "gcp" | "google" | "gcs" => AssetBackend::Gcp,
        "azure" => AssetBackend::Azure,
        _ => AssetBackend::Fs,
    };
    Config::new(&name, app_secret, launch_mode, asset_backend)
}

async fn build_services() -> ServiceLayer {
    let openai_api_key = env::var("OPENAI_API_KEY").expect("invalid or missing openai api key");
    let openai_base_uri = env::var("OPENAI_BASE_URI").expect("invalid openai base uri");

    let openai_client = OpenAIClient::new(&openai_api_key, &openai_base_uri);

    let auth: Box<dyn Authenticator> = match env::var("AUTH_PROVIDER")
        .expect("missing auth provider")
        .as_str()
    {
        "auth0" => {
            let tenant_uri = env::var("AUTH0_TENANT").expect("missing auth0 tenant uri");
            let aud_raw = env::var("AUTH0_AUDIENCES").expect("missing auth0 audience(s) value");

            let audiences: Vec<String> = aud_raw
                .split_ascii_whitespace()
                .map(|aud| aud.into())
                .collect();

            Box::new(
                Auth0::new(&tenant_uri, audiences)
                    .await
                    .expect("error initializing auth0 provider"),
            )
        }
        "noop" => Box::new(NoOpAuth::new()),
        _ => unimplemented!(),
    };

    ServiceLayer::new(openai_client, auth)
}

async fn build_storage_layer() -> StorageLayer {
    let db_url = env::var("DATABASE_URL").expect("missing database url");
    let cache_url = env::var("CACHE_URL").expect("missing cache url");

    let sql = sql::create_pool(&db_url)
        .await
        .expect("error initializing database connection pool");
    let cache = cache::create_pool(&cache_url)
        .await
        .expect("error initializing cache connection pool");

    StorageLayer::new(sql, cache)
}

async fn build_app_state() -> AppState {
    dotenvy::dotenv().expect("error loading environment variables");

    let config = build_config();
    let storage_layer = build_storage_layer().await;
    let services = build_services().await;

    AppState::new(config, storage_layer, services)
}

pub async fn build_app() -> Router {
    let shared_state = Arc::new(build_app_state().await);
    let api_routes = api::routes(shared_state.clone());

    sqlx::migrate!("./migrations")
        .run(&shared_state.storage_layer.sql)
        .await
        .expect("failed to run database migrations");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Router::new()
        .with_state(shared_state)
        .nest("/api", api_routes)
        .fallback(|| async move {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"status": "Not Found"})),
            )
        })
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
