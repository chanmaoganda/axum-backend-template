use axum::Router;

use crate::config::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}
