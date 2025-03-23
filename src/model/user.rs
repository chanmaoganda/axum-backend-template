use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::{alias::{MyResult, RedisConn, SqlxPool}, config::constants, error::MyError};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub nickname: Option<String>,
}

impl User {
    pub async fn check_user_exists(conn: &mut RedisConn, username: &str) -> MyResult<bool> {
        let exists: bool = conn
            .sismember(constants::USER_LIST, username)
            .await
            .map_err(|error| {
                crate::errlog!(error);
                MyError::RedisError
            })?;
        Ok(exists)
    }

    pub async fn find_db_by_name(
        pool: &SqlxPool,
        name: &str
    ) -> MyResult<User> {
        tracing::debug!(
            target = crate::config::targets::PARAMS,
            "querying user with name [{}]",
            name
        );

        let result = sqlx::query_as!(
                Self,
                "SELECT username, email, nickname FROM users WHERE username = $1",
                name
            )
            .fetch_one(pool)
            .await
            .map_err(|error| {
                crate::errlog!(error);
                MyError::DatabaseError
            })?;

        Ok(result)
    }
}
