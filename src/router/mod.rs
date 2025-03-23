use axum::{Router, routing::get};
use axum_prometheus::PrometheusMetricLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::config::app_state::AppState;

pub mod admin;
pub mod v0;

pub fn router() -> Router<AppState> {
    tracing::debug!("registering for global service");

    let cors_layer = CorsLayer::new().allow_origin(Any); // Allow all origins (open policy)

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
        .nest("/v0", v0::router())
        .nest("/admin", admin::router())
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(cors_layer)
        .layer(prometheus_layer)
}
