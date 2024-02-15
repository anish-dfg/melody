use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DbError {
    #[error("invalid database uri. error: {0}")]
    InvalidUri(String),
    #[error("unable to create connection pool. error: {0}")]
    PoolCreate(String),
    #[error("unable to create client. error: {0}")]
    CreateClient(String),
    #[error("invalid value for cache (not a supported type). error: {0}")]
    InvalidCacheValue(String),
    #[error("unable to parse into value. error: {0}")]
    Parse(String),
    #[error("internal server error with database client. error: {0}")]
    ServerError(String),
}
