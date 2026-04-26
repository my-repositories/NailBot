use nailbot::{
    api::run_api,
    bot::build_bot,
    bot::reminders::start_reminder_service,
    shared::{
        config::{Mode, Settings},
        tenant::TenantContext,
    },
};
use nailbot::api::database::init_db;
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
    println!("NailBot started");

    let pool = init_db(&settings.database_url).await?;
    let api_bind = std::env::var("API_BIND").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let bind_addr: std::net::SocketAddr = api_bind.parse()?;
    let api_pool = pool.clone();
    tokio::spawn(async move {
        if let Err(err) = run_api(api_pool, bind_addr).await {
            tracing::error!("api server failed: {err}");
        }
    });

    if !settings.bot_token.is_empty() {
        let bot = build_bot(&settings.bot_token);
        start_reminder_service(bot, pool);
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}
