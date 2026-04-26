use nailbot::{
    bot::build_bot,
    config::{Mode, Settings},
    database::init_db,
    reminders::start_reminder_service,
    tenant::TenantContext,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let settings = Settings::from_env()?;
    let tenant_context = match settings.mode {
        Mode::Saas => TenantContext {
            client_id: 0,
            mode: Mode::Saas,
        },
        Mode::OnPrem => TenantContext::onprem(),
    };

    if matches!(settings.mode, Mode::OnPrem) && settings.license_key.is_none() {
        anyhow::bail!("onprem mode requires LICENSE_KEY");
    }

    info!(
        mode = ?settings.mode,
        client_id = tenant_context.client_id,
        "NailBot started"
    );

    let pool = init_db(&settings.database_url).await?;
    if !settings.bot_token.is_empty() {
        let bot = build_bot(&settings.bot_token);
        start_reminder_service(bot, pool);
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}
