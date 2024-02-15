use jsonwebtoken::{jwk::JwkSet, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    authenticator::{Authenticator, UserData},
    errors::AuthError,
};

const AUTHENTICATOR_ID: &'static str = "Auth0";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auth0Configuration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub device_authorization_endpoint: String,
    pub userinfo_endpoint: String,
    pub mfa_challenge_endpoint: String,
    pub jwks_uri: String,
    pub registration_endpoint: String,
    pub revocation_endpoint: String,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<String>,
    pub response_modes_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub request_uri_parameter_supported: bool,
    pub request_parameter_supported: bool,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<String>,
    pub backchannel_logout_supported: bool,
    pub backchannel_logout_session_supported: bool,
    pub end_session_endpoint: String,
}

#[derive(Debug, Clone)]
pub struct Auth0 {
    pub tenant_base_uri: String,
    pub configuration: Auth0Configuration,
}

impl Auth0 {
    pub async fn new(tenant_base_uri: &str) -> Result<Self, AuthError> {
        let discovery_endpoint = tenant_base_uri.to_owned() + "/.well-known/openid-configuration";
        let res = reqwest::get(&discovery_endpoint)
            .await
            .map_err(|e| AuthError::Init(AUTHENTICATOR_ID.into(), e.to_string()))?;
        let configuration: Auth0Configuration = res
            .json()
            .await
            .map_err(|e| AuthError::Init(AUTHENTICATOR_ID.into(), e.to_string()))?;
        Ok(Auth0 {
            tenant_base_uri: tenant_base_uri.into(),
            configuration,
        })
    }
}

pub struct Auth0UserData {}

#[async_trait::async_trait]
impl Authenticator for Auth0 {
    async fn authenticate(&self, token: &str) -> Result<UserData, AuthError> {
        let jwks_uri: &str = &self.configuration.jwks_uri;
        let res = reqwest::get(jwks_uri)
            .await
            .map_err(|e| AuthError::FetchJwks(AUTHENTICATOR_ID.into(), e.to_string()))?;
        let jwks: JwkSet = res
            .json()
            .await
            .map_err(|e| AuthError::FetchJwks(AUTHENTICATOR_ID.into(), e.to_string()))?;
        let header = jsonwebtoken::decode_header(token)
            .map_err(|e| AuthError::MalformedToken(AUTHENTICATOR_ID.into(), e.to_string()))?;

        // Get the jwk with the matching kid if it exists
        //
        match header.kid {
            Some(kid) => {
                use jsonwebtoken::jwk::AlgorithmParameters;
                match jwks
                    .keys
                    .into_iter()
                    .find(|jwk| match jwk.common.key_id.clone() {
                        Some(jwk_id) => jwk_id == kid,
                        None => false,
                    }) {
                    Some(jwk) => match jwk.algorithm {
                        AlgorithmParameters::EllipticCurve(_) => {
                            Err(AuthError::UnimplementedAlgorithm(
                                AUTHENTICATOR_ID.into(),
                                "elliptic curve".into(),
                            ))?
                        }
                        AlgorithmParameters::RSA(rsa) => {
                            let (n, e) = (rsa.n, rsa.e);

                            let decoded = jsonwebtoken::decode::<Value>(
                                token,
                                &DecodingKey::from_rsa_components(&n, &e).map_err(|e| {
                                    AuthError::InvalidToken(AUTHENTICATOR_ID.into(), e.to_string())
                                })?,
                                &Validation::new(header.alg),
                            )
                            .map_err(|e| {
                                AuthError::InvalidToken(AUTHENTICATOR_ID.into(), e.to_string())
                            })?;

                            println!("{decoded:#?}");

                            Ok(UserData::Auth0(Auth0UserData {}))
                        }
                        AlgorithmParameters::OctetKey(_) => {
                            Err(AuthError::UnimplementedAlgorithm(
                                AUTHENTICATOR_ID.into(),
                                "octet key".into(),
                            ))?
                        }
                        AlgorithmParameters::OctetKeyPair(_) => {
                            Err(AuthError::UnimplementedAlgorithm(
                                AUTHENTICATOR_ID.into(),
                                "octet key pair".into(),
                            ))?
                        }
                    },
                    None => Err(AuthError::NoMatchingKey(
                        AUTHENTICATOR_ID.into(),
                        format!("no jwk with key id {kid} found").into(),
                    )),
                }
            }
            None => Err(AuthError::InvalidToken(
                AUTHENTICATOR_ID.into(),
                "Missing key id (kid) in token header".into(),
            )),
        }
    }
}
