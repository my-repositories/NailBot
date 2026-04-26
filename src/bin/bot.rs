use nailbot::api::database::init_db;
use nailbot::bot::{build_bot, reminders::start_reminder_service};
use nailbot::shared::config::{Mode, Settings};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let settings = Settings::from_env()?;
    if matches!(settings.mode, Mode::OnPrem) && settings.license_key.is_none() {
        anyhow::bail!("onprem mode requires LICENSE_KEY");
    }

    if settings.bot_token.is_empty() {
        warn!("BOT_TOKEN is empty; bot service will not start");
        return Ok(());
    }

    let pool = init_db(&settings.database_url).await?;
    let bot = build_bot(&settings.bot_token);
    start_reminder_service(bot, pool);

    info!("Bot service started");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
