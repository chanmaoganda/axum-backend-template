use axum::{Router, routing::get};

use crate::{config::app_state::AppState, service};

pub fn router() -> Router<AppState> {
    tracing::debug!("registering for v0 service");
    Router::new().route(
        "/user/{name}",
        get(service::user::user_service),
    )
}
