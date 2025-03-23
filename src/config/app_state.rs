use std::{env, sync::Arc};

use axum::extract::FromRef;
use redis::aio::ConnectionManager;

use crate::{
    alias::{RedisConn, SqlxPool, MyResult},
    error::MyError,
};

#[derive(Clone)]
pub struct Pools {
    pub sqlx_pool: Arc<crate::alias::SqlxPool>,
    /// since redis::aio::ConnectionManager has a inner Arc ref, no need for Arc wrapped here
    pub redis_conn: crate::alias::RedisConn,
}

#[derive(Clone)]
pub struct AppState {
    pools: Pools,
}

impl AppState {
    pub async fn new() -> MyResult<Self> {
        tracing::debug!(
            target = crate::config::targets::BOOTSTRAP,
            "creating AppState"
        );

        let sqlx_pool = sqlx_pool().await.map_err(|error| {
            crate::errlog!(error);
            MyError::DatabaseError
        })?;

        let sqlx_pool = Arc::new(sqlx_pool);

        let redis_conn = redis_client().await.map_err(|error| {
            crate::errlog!(error);
            MyError::RedisError
        })?;

        let pools = Pools {
            sqlx_pool,
            redis_conn,
        };

        Ok(AppState { pools })
    }
}

impl FromRef<AppState> for Arc<SqlxPool> {
    fn from_ref(input: &AppState) -> Self {
        input.pools.sqlx_pool.clone()
    }
}

impl FromRef<AppState> for RedisConn {
    fn from_ref(input: &AppState) -> Self {
        input.pools.redis_conn.clone()
    }
}

impl FromRef<AppState> for Pools {
    fn from_ref(input: &AppState) -> Self {
        input.pools.clone()
    }
}

pub async fn sqlx_pool() -> MyResult<crate::alias::SqlxPool, sqlx::Error> {
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(error) => {
            crate::errlog!(error);
            panic!("DATABASE_URL error: {}", error);
        }
    };
    tracing::info!(
        target = crate::config::targets::BOOTSTRAP,
        "Connecting to Database [{}]",
        db_url
    );
    SqlxPool::connect(&db_url).await
}

pub async fn redis_client() -> MyResult<redis::aio::ConnectionManager, redis::RedisError> {
    let redis_url = match env::var("REDIS_URL") {
        Ok(url) => url,
        Err(error) => {
            crate::errlog!(error);
            panic!("REDIS_URL error: {}", error);
        }
    };
    tracing::info!(
        target = crate::config::targets::BOOTSTRAP,
        "Connecting to Redis [{}]",
        redis_url
    );
    let client = redis::Client::open(redis_url)?;
    ConnectionManager::new(client).await
}
