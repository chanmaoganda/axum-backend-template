use redis::AsyncCommands;

use crate::{alias::{MyResult, RedisConn, SqlxPool}, config::constants, error::MyError, model::user::User};

pub mod app;

pub async fn load_users(pool: &SqlxPool, conn: &mut RedisConn) -> MyResult<()> {
    let users: Vec<String> = sqlx::query_as!(
        User,
        "SELECT username, email, nickname FROM users",
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        crate::errlog!(error);
        MyError::DatabaseError
    })?
    .into_iter()
    .map(|user| user.username)
    .collect();

    let res = conn.sadd(constants::USER_LIST, &users).await.map_err(|error| {
        crate::errlog!(error);
        MyError::RedisError
    })?;

    tracing::info!(
        target = crate::config::targets::BOOTSTRAP,
        "user list loaded"
    );

    Ok(res)
}
