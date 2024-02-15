use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use super::errors::DbError;

const MAX_CONNECTIONS: u32 = 100;

///
///
/// * `max_connections`: The maximum number of connections the pool should maintain at once
pub async fn create_pool(db_url: &str) -> Result<Pool<Postgres>, DbError> {
    // let db_url = env::var("DATABASE_URL").map_err(|e| DbError::InvalidUri(e.to_string()))?;

    PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(db_url)
        .await
        .map_err(|e| DbError::PoolCreate(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::util;

    #[tokio::test]
    pub async fn test_create_pool() {
        util::test_util::init();

        let db_url = env::var("DATABASE_URL").expect("invalid or missing database url");

        let _ = create_pool(&db_url).await.expect("error creating pool");
    }
}
