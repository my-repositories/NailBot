use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Message, Requester},
    types::UserId,
    utils::command::BotCommands,
};
use tracing::info;
use crate::bot::states::SessionStore;

pub mod handlers;
pub mod keyboards;
pub mod reminders;
pub mod states;

pub fn build_bot(token: &str) -> Bot {
    Bot::new(token)
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands")]
enum Command {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "show help")]
    Help,
    #[command(description = "start appointment flow")]
    Appointment,
}

pub async fn run_polling(bot: Bot, pool: PgPool, default_locale: String, client_id: i64) {
    let sessions: Arc<SessionStore> = Arc::new(tokio::sync::RwLock::new(HashMap::new()));
    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let pool = pool.clone();
        let default_locale = default_locale.clone();
        let sessions = sessions.clone();
        async move {
            if let Some(text) = msg.text() {
                let user_id: Option<UserId> = msg.from.as_ref().map(|u| u.id);
                let Some(user_id) = user_id else {
                    return Ok(());
                };

                if let Ok(cmd) = Command::parse(text, "nailbot") {
                    match cmd {
                        Command::Start => {
                            let message = handlers::user::start_message_for_user(
                                &pool,
                                client_id,
                                user_id,
                                &default_locale,
                            )
                            .await;
                            let locale =
                                handlers::resolve_user_locale(&pool, client_id, user_id, &default_locale)
                                    .await;
                            bot.send_message(msg.chat.id, message)
                                .reply_markup(keyboards::main_menu(&locale))
                                .await?;
                        }
                        Command::Help => {
                            let message = handlers::user::help_message_for_user(
                                &pool,
                                client_id,
                                user_id,
                                &default_locale,
                            )
                            .await;
                            bot.send_message(msg.chat.id, message).await?;
                        }
                        Command::Appointment => {
                            handlers::user::start_booking(&sessions, client_id, user_id).await;
                            let prompt = handlers::user::name_prompt_for_user(
                                &pool,
                                client_id,
                                user_id,
                                &default_locale,
                            )
                            .await;
                            bot.send_message(msg.chat.id, prompt).await?;
                        }
                    }
                }
            }
            Ok(())
        }
    })
    .await;
    info!("Polling loop stopped");
}
