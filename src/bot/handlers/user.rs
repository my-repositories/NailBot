use chrono::{NaiveDate, NaiveTime, Utc};
use regex::Regex;
use teloxide::types::UserId;
use tracing::info;

use crate::bot::states::{AppointmentState, SessionStore, UserSession};
use crate::shared::models::AppointmentDraft;

pub fn start_message() -> &'static str {
    "👋 Welcome to NailBot! How can I help you?"
}

pub fn help_message() -> &'static str {
    "📚 Help:\n\n/start — Show main menu\n/help — Show this help\n/appointment — Book a new appointment\n\nUse the menu below to navigate:"
}

pub async fn start_booking(sessions: &SessionStore, client_id: i64, user_id: UserId) {
    let mut store = sessions.write().await;
    store.insert(
        (client_id, user_id),
        UserSession {
            state: AppointmentState::SelectingDate,
            appointment_data: AppointmentDraft::default(),
            last_activity: Utc::now(),
        },
    );
    info!("State changed: Idle -> SelectingDate client_id={client_id} user_id={user_id}");
}

pub async fn set_date(
    sessions: &SessionStore,
    client_id: i64,
    user_id: UserId,
    date: NaiveDate,
) {
    if let Some(session) = sessions.write().await.get_mut(&(client_id, user_id)) {
        session.appointment_data.date = Some(date);
        session.state = AppointmentState::SelectingTime;
        session.last_activity = Utc::now();
    }
}

pub fn validate_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty() && trimmed.len() <= 50
}

pub fn name_prompt() -> &'static str {
    "Please enter your full name:"
}

pub fn phone_prompt() -> &'static str {
    "Please enter your phone number:"
}

pub fn back_button_label() -> &'static str {
    "🔙 Back"
}

pub fn validate_phone(phone: &str) -> bool {
    Regex::new(r"^[+0-9()\s\-]{10,25}$")
        .map(|r| r.is_match(phone))
        .unwrap_or(false)
}

pub fn redact_phone(phone: &str) -> String {
    let digits = phone.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
    let suffix = digits
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("+***-***-***-{suffix}")
}

pub fn parse_time(time: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(time, "%H:%M").ok()
}
