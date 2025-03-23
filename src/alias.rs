pub type SqlxPool = sqlx::PgPool;
pub type RedisConn = redis::aio::ConnectionManager;
pub type MyResult<T, E = crate::error::MyError> = Result<T, E>;
