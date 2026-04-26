use nailbot::bot::{build_bot, run_polling};
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

    let bot = build_bot(&settings.bot_token);
    info!("Bot service started");

    let client_id = if matches!(settings.mode, Mode::OnPrem) { 1 } else { 0 };
    run_polling(
        bot,
        settings.api_base_url.clone(),
        settings.default_locale.clone(),
        client_id,
    )
    .await;
    Ok(())
}
