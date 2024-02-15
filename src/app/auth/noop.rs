use super::{
    authenticator::{Authenticator, UserData},
    errors::AuthError,
};

pub struct NoOpAuthenticator {}

pub struct NoOpUserData {}

#[async_trait::async_trait]
impl Authenticator for NoOpAuthenticator {
    #[allow(unused_variables)] // no op authenticator will not use the token
    async fn authenticate(&self, token: &str) -> Result<UserData, AuthError> {
        Ok(UserData::NoOp(NoOpUserData {}))
    }
}
