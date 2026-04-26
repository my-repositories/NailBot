use teloxide::Bot;

pub mod handlers;
pub mod keyboards;
pub mod reminders;
pub mod states;

pub fn build_bot(token: &str) -> Bot {
    Bot::new(token)
}
