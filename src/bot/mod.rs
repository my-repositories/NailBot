use teloxide::Bot;

pub fn build_bot(token: &str) -> Bot {
    Bot::new(token)
}
