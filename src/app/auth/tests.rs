use std::env;

use crate::app::{auth::authenticator::Authenticator, util::test_util};

use super::auth0::Auth0;

#[tokio::test]
pub async fn test_build_auth0_provider() {
    test_util::init();

    let tenant_uri =
        env::var("AUTH0_TENANT").expect("invalid or missing AUTH0_TENANT environment variable");

    let auth0 = Auth0::new(&tenant_uri, vec![])
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    println!("{auth0:?}");
}

#[tokio::test]
pub async fn test_auth0_authenticate() {
    test_util::init();

    let tenant_uri =
        env::var("AUTH0_TENANT").expect("invalid or missing AUTH0_TENANT environment variable");

    let auth0 = Auth0::new(&tenant_uri, vec![])
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    auth0.authenticate("").await.unwrap();
}
