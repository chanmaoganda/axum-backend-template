use std::net::SocketAddr;

use axum::Router;

use crate::{config::app_state::AppState, router, util};

pub async fn start() -> std::io::Result<()> {
    //TODO: use clap here???
    util::check_env();

    // util::tracer::prompt_tracer();
    let _guard = util::tracer::file_tracer();

    let app_state = AppState::new().await.expect("AppState Initialize Error");

    let router = Router::new().merge(router::router()).with_state(app_state);

    let address = SocketAddr::from(([0, 0, 0, 0], 8080));

    tracing::info!(
        target = crate::config::targets::BOOTSTRAP,
        "Server listening on {}",
        address
    );
    axum_server::bind(address)
        .serve(router.into_make_service())
        .await
}
