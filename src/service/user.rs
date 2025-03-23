use axum::{extract::{Path, State}, Json};

use crate::{
    alias::MyResult,
    config::app_state::Pools, model::user::User,
};

pub async fn user_service(
    State(pools): State<Pools>,
    Path(username): Path<String>,
) -> MyResult<Json<bool>> {
    tracing::debug!("Receiving Query for {}", username);

    let mut conn: redis::aio::ConnectionManager = pools.redis_conn;

    if let Ok(exists) = User::check_user_exists(&mut conn, &username).await {
        tracing::debug!("Cache {} Hit!", username);
        return Ok(Json(exists));
    }

    Ok(Json(false))
}
