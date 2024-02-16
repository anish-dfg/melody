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
    errors::LaunchError,
    launch::LaunchMode,
    state::{AppState, Config, ServiceLayer, StorageLayer},
};
use std::{env, sync::Arc};

/// Create app configuration
fn build_config() -> Config {
    let name = env::var("APP_NAME")
        .map_err(|e| LaunchError::InvalidName(e.to_string()))
        .unwrap();
    let app_secret = env::var("APP_SECRET")
        .map(|s| s.as_bytes().to_vec())
        .map_err(|e| LaunchError::InvalidSecret(e.to_string()))
        .unwrap();
    let launch_mode = match env::var("LAUNCH_MODE")
        .map_err(|e| LaunchError::InvalidAssetBackend(e.to_string()))
        .unwrap()
        .as_str()
    {
        "development" => LaunchMode::Development,
        "testing" => LaunchMode::Testing,
        "staging" => LaunchMode::Staging,
        _ => LaunchMode::Production,
    };
    let asset_backend = match env::var("ASSET_BACKEND")
        .map_err(|e| LaunchError::InvalidSessionInterface(e.to_string()))
        .unwrap()
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
    let openai_api_key = env::var("OPENAI_API_KEY")
        .map_err(|e| LaunchError::InvalidOpenAIApiKey(e.to_string()))
        .unwrap();

    let openai_client = OpenAIClient::new(&openai_api_key, "https://api.openai.com/v2");

    let auth: Box<dyn Authenticator> = match env::var("AUTH_PROVIDER")
        .map_err(|e| LaunchError::InvalidAuthProvider("".into(), e.to_string()))
        .unwrap()
        .as_str()
    {
        "auth0" => {
            let tenant_uri = env::var("AUTH0_TENANT")
                .map_err(|e| LaunchError::InvalidAuth0Tenant(e.to_string()))
                .unwrap();

            let aud_raw = env::var("AUTH0_AUDIENCES")
                .map_err(|e| LaunchError::InitAuth("Auth0".into(), e.to_string()))
                .unwrap();

            let audiences: Vec<String> =
                aud_raw.split_ascii_whitespace().map(|a| a.into()).collect();

            Box::new(
                Auth0::new(&tenant_uri, audiences)
                    .await
                    .map_err(|e| LaunchError::InitAuth("Auth0".into(), e.to_string()))
                    .unwrap(),
            )
        }
        "noop" => Box::new(NoOpAuth::new()),
        _ => unimplemented!(),
    };

    ServiceLayer::new(openai_client, auth)
}

async fn build_storage_layer() -> StorageLayer {
    let db_url = env::var("DATABASE_URL")
        .map_err(|e| LaunchError::InvalidDatabaseUrl(e.to_string()))
        .unwrap();
    let cache_url = env::var("CACHE_URL")
        .map_err(|e| LaunchError::InvalidCacheUrl(e.to_string()))
        .unwrap();

    let sql = sql::create_pool(&db_url)
        .await
        .map_err(|e| LaunchError::InitSql(e.to_string()))
        .unwrap();
    let cache = cache::create_pool(&cache_url)
        .await
        .map_err(|e| LaunchError::InitCache(e.to_string()))
        .unwrap();

    StorageLayer::new(sql, cache)
}

async fn build_app_state() -> AppState {
    dotenvy::dotenv()
        .map_err(|e| LaunchError::LoadEnvironment(e.to_string()))
        .unwrap();

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
        .map_err(|e| LaunchError::Migrate(e.to_string()))
        .unwrap();

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
