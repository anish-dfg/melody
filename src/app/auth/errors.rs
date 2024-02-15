use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("unable to initialize {0} client. error: {1}")]
    Init(String, String),
    #[error("unable to fetch jwks for {0} client. error: {1}")]
    FetchJwks(String, String),
    #[error("{0} client detected malformed token. error: {1}")]
    MalformedToken(String, String),
    #[error("{0} client detected invalid token. error: {1}")]
    InvalidToken(String, String),
    #[error("no matching jwk found ({0} client). error: {1}")]
    NoMatchingKey(String, String),
    #[error("{0} client does not implement {1} algorithm")]
    UnimplementedAlgorithm(String, String),
}
