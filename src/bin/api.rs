use nailbot::api::database::init_db;
use nailbot::api::run_api;
use nailbot::shared::config::Settings;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let settings = Settings::from_env()?;
    let pool = init_db(&settings.database_url).await?;
    let api_bind = std::env::var("API_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let bind_addr: std::net::SocketAddr = api_bind.parse()?;

    info!(bind = %bind_addr, "Starting API service");
    run_api(pool, bind_addr).await
}
