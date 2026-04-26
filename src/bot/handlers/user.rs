use chrono::{NaiveDate, NaiveTime, Utc};
use regex::Regex;
use sqlx::PgPool;
use teloxide::types::UserId;
use tracing::info;

use crate::bot::states::{AppointmentState, SessionStore, UserSession};
use crate::shared::i18n::tr;
use crate::shared::models::AppointmentDraft;
use super::resolve_user_locale;

pub fn start_message(locale: &str) -> String {
    tr(locale, "start-message")
}

pub fn help_message(locale: &str) -> String {
    tr(locale, "help-message")
}

pub async fn start_message_for_user(
    pool: &PgPool,
    client_id: i64,
    user_id: UserId,
    default_locale: &str,
) -> String {
    let locale = resolve_user_locale(pool, client_id, user_id, default_locale).await;
    start_message(&locale)
}

pub async fn help_message_for_user(
    pool: &PgPool,
    client_id: i64,
    user_id: UserId,
    default_locale: &str,
) -> String {
    let locale = resolve_user_locale(pool, client_id, user_id, default_locale).await;
    help_message(&locale)
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

pub fn name_prompt(locale: &str) -> String {
    tr(locale, "name-prompt")
}

pub async fn name_prompt_for_user(
    pool: &PgPool,
    client_id: i64,
    user_id: UserId,
    default_locale: &str,
) -> String {
    let locale = resolve_user_locale(pool, client_id, user_id, default_locale).await;
    name_prompt(&locale)
}

pub fn phone_prompt(locale: &str) -> String {
    tr(locale, "phone-prompt")
}

pub async fn phone_prompt_for_user(
    pool: &PgPool,
    client_id: i64,
    user_id: UserId,
    default_locale: &str,
) -> String {
    let locale = resolve_user_locale(pool, client_id, user_id, default_locale).await;
    phone_prompt(&locale)
}

pub fn back_button_label(locale: &str) -> String {
    tr(locale, "back-button")
}

pub async fn back_button_label_for_user(
    pool: &PgPool,
    client_id: i64,
    user_id: UserId,
    default_locale: &str,
) -> String {
    let locale = resolve_user_locale(pool, client_id, user_id, default_locale).await;
    back_button_label(&locale)
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
