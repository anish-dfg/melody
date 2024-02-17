use reqwest::Client;
use sqlx::PgPool;

use crate::{
    app::{
        auth::authenticator::Authenticator, openai::OpenAIClient, storage::cache::RedisPool,
        types::AssetBackend,
    },
    launch::LaunchMode,
};

#[derive(Clone)]
pub struct Config {
    pub name: String,
    pub secret: Vec<u8>,
    pub launch_mode: LaunchMode,
    pub asset_backend: AssetBackend,
}

impl Config {
    pub fn new(
        name: &str,
        secret: Vec<u8>,
        launch_mode: LaunchMode,
        asset_backend: AssetBackend,
    ) -> Self {
        Self {
            name: name.to_owned(),
            secret,
            launch_mode,
            asset_backend,
        }
    }
}

#[derive(Clone)]
pub struct StorageLayer {
    pub sql: PgPool,
    pub cache: RedisPool,
}

// #[derive(Clone)]
pub struct ServiceLayer {
    pub ai: OpenAIClient,
    pub http: Client,
    pub auth: Box<dyn Authenticator>,
}

impl ServiceLayer {
    pub fn new(ai: OpenAIClient, auth: Box<dyn Authenticator>) -> Self {
        let http = Client::new();
        Self { ai, http, auth }
    }
}

impl StorageLayer {
    pub fn new(sql: PgPool, cache: RedisPool) -> Self {
        Self { sql, cache }
    }
}

// #[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub storage_layer: StorageLayer,
    pub services: ServiceLayer,
}

impl AppState {
    pub fn new(config: Config, storage_layer: StorageLayer, service_layer: ServiceLayer) -> Self {
        Self {
            config,
            storage_layer,
            services: service_layer,
        }
    }
}
