use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};
use tracing_subscriber::{EnvFilter, fmt::time};

pub fn prompt_tracer() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(time::LocalTime::rfc_3339())
        .init();
}

pub fn file_tracer() -> WorkerGuard {
    let file_append = RollingFileAppender::builder()
        .filename_prefix("axum")
        .filename_suffix("log")
        .rotation(Rotation::DAILY)
        .build("./logs")
        .unwrap();

    let (non_blocking, guard) = tracing_appender::non_blocking(file_append);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_timer(time::LocalTime::rfc_3339())
        .with_writer(non_blocking)
        .init();

    guard
}
