use mobc::{Connection, Pool};
use mobc_redis::redis::AsyncCommands;
use mobc_redis::{redis, RedisConnectionManager};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::Duration;

use super::errors::DbError;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConn = Connection<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub async fn create_pool(cache_url: &str) -> Result<RedisPool, DbError> {
    let client =
        redis::Client::open(cache_url).map_err(|e| DbError::CreateClient(e.to_string()))?;
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager);
    Ok(pool)
}

pub async fn set_json<'a, T>(conn: &mut RedisConn, key: &str, value: T) -> Result<(), DbError>
where
    T: Serialize + Deserialize<'a>,
{
    let bytes = serde_json::to_string(&value)
        .map(|s| s.as_bytes().to_vec())
        .map_err(|e| DbError::InvalidCacheValue(e.to_string()))?;

    conn.set(key.to_owned(), bytes)
        .await
        .map_err(|e| DbError::ServerError(e.to_string()))?;

    Ok(())
}

pub async fn get_json<'a, T>(conn: &mut RedisConn, key: &str) -> Result<Option<T>, DbError>
where
    T: Serialize + DeserializeOwned,
{
    let json: Option<String> = conn
        .get(key)
        .await
        .map_err(|e| DbError::ServerError(e.to_string()))?;
    match json {
        Some(value) => {
            let s: T = serde_json::from_str(&value).map_err(|e| DbError::Parse(e.to_string()))?;
            Ok(Some(s))
        }
        None => Ok(None),
    }
}

pub async fn evict(
    conn: &mut Connection<RedisConnectionManager>,
    key: &str,
) -> Result<(), DbError> {
    conn.del(key)
        .await
        .map_err(|e| DbError::ServerError(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::app::{util, util::test_util::TestStruct};

    use super::*;

    #[tokio::test]
    pub async fn test_create_pool() {
        util::test_util::init();
        let cache_url = env::var("CACHE_URL").expect("invalid or missing cache url");

        let _ = create_pool(&cache_url).await.expect("error creating pool");
    }

    #[tokio::test]
    pub async fn test_get_set_evict() {
        util::test_util::init();

        let cache_url = env::var("CACHE_URL").expect("invalid or missing cache url");

        let pool = create_pool(&cache_url).await.expect("error creating pool");
        let mut conn = pool.get().await.expect("error acquiring connection");

        let t = TestStruct {
            first_name: "Jenny".to_owned(),
            last_name: "Cho".to_owned(),
        };

        set_json(&mut conn, "test", t)
            .await
            .expect("error setting key");

        let _: TestStruct = get_json(&mut conn, "test")
            .await
            .expect("error retrieving value")
            .unwrap();

        evict(&mut conn, "test").await.expect("error deleting key");

        let deleted: Option<TestStruct> = get_json(&mut conn, "test")
            .await
            .expect("error retrieving value");

        assert!(deleted.is_none());
    }
}
