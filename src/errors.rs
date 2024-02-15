use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LaunchError {
    #[error("invalid or missing app secret. error: {0}")]
    InvalidSecret(String),
    #[error("invalid or missing database url. error: {0}")]
    InvalidDatabaseUrl(String),
    #[error("invalid or missing cache url. error: {0}")]
    InvalidCacheUrl(String),
    #[error("invalid or missing openai api key. error: {0}")]
    InvalidOpenAIApiKey(String),
    #[error("error initializing database connection pool. error: {0}")]
    InitSql(String),
    #[error("error initializing cache pool. error: {0}")]
    InitCache(String),
    #[error("invalid or missing session interface. error: {0}")]
    InvalidSessionInterface(String),
    #[error("invalid or missing app name. error: {0}")]
    InvalidName(String),
    #[error("invalid or missing asset backend. error: {0}")]
    InvalidAssetBackend(String),
    #[error("could not load environment variables. error: {0}")]
    LoadEnvironment(String),
    #[error("could not run database migrations. error: {0}")]
    Migrate(String),
    #[error("invalid authentication provider {0}. error: {1}")]
    InvalidAuthProvider(String, String),
    #[error("invalid auth0 tenant. error: {0}")]
    InvalidAuth0Tenant(String),
    #[error("error initializing {0} authentication provider. error: {1}")]
    InitAuth(String, String),
    #[error("unknown internal server failure. error: {0}")]
    Internal(String),
}
