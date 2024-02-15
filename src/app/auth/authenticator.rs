use super::{auth0::Auth0UserData, errors::AuthError};

pub enum UserData {
    Auth0(Auth0UserData),
    Keycloak,
    Okta,
}

#[async_trait::async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self, token: &str) -> Result<UserData, AuthError>;
}
