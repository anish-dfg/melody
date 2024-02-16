use super::{
    authenticator::{Authenticator, UserData},
    errors::AuthError,
};

pub struct NoOpAuth {}

impl NoOpAuth {
    pub fn new() -> Self {
        NoOpAuth {}
    }
}

pub struct NoOpUserData {}

#[async_trait::async_trait]
impl Authenticator for NoOpAuth {
    #[allow(unused_variables)] // no op authenticator will not use the token
    async fn authenticate(&self, token: &str) -> Result<UserData, AuthError> {
        Ok(UserData::NoOp(NoOpUserData {}))
    }
}
