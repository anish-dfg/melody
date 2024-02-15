use super::{auth0::Auth0UserData, errors::AuthError, noop::NoOpUserData};

pub enum UserData {
    Auth0(Auth0UserData),
    Keycloak,
    Okta,
    NoOp(NoOpUserData),
}

#[async_trait::async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, token: &str) -> Result<UserData, AuthError>;
}
