#[tokio::main]
async fn main() -> std::io::Result<()> {
    axum_backend_template::bootstrap::app::start().await
}
