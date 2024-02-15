use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "hash_algorithm")]
#[sqlx(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Argon2,
    Bcrypt,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy)]
#[sqlx(type_name = "asset_backend")]
#[sqlx(rename_all = "lowercase")]
pub enum AssetBackend {
    Fs,
    Aws,
    Gcp,
    Azure,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy)]
#[sqlx(type_name = "asset_visibility")]
#[sqlx(rename_all = "lowercase")]
pub enum AssetVisibility {
    Public,
    Private,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy)]
#[sqlx(type_name = "auth_method")]
pub enum AuthMethod {
    #[sqlx(rename = "first_party")]
    FirstParty,
    #[sqlx(rename = "third_party")]
    ThirdParty,
}
