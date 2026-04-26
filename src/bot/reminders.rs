use chrono::{Duration, NaiveDateTime, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use teloxide::payloads::SendMessageSetters;
use teloxide::{Bot, requests::Requester};
use tracing::{error, info};

use crate::api::database::queries;
use crate::bot::keyboards::reminder_keyboard;
use crate::shared::i18n::{normalize_locale, tr_with_args};
use crate::shared::models::Appointment;

pub fn start_reminder_service(bot: Bot, pool: PgPool) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1800));
        loop {
            interval.tick().await;
            if let Err(err) = check_and_send_reminders(&bot, &pool).await {
                error!("reminder job error: {err}");
            }
        }
    });
}

pub async fn check_and_send_reminders(bot: &Bot, pool: &PgPool) -> Result<(), sqlx::Error> {
    let appointments: Vec<Appointment> = sqlx::query_as(
        "SELECT id, client_id, user_id, appointment_date, appointment_time, service_type, status, created_at
         FROM appointments WHERE status = 'confirmed'",
    )
    .fetch_all(pool)
    .await?;

    let now = Utc::now().naive_utc();
    let default_locale = normalize_locale(
        &std::env::var("DEFAULT_LOCALE").unwrap_or_else(|_| "en".to_string()),
    )
    .to_string();
    for appt in appointments {
        let dt = NaiveDateTime::new(appt.appointment_date, appt.appointment_time);
        let diff = dt - now;
        let is_24h = diff >= Duration::hours(23) + Duration::minutes(55)
            && diff <= Duration::hours(24) + Duration::minutes(5);
        let is_1h = diff >= Duration::minutes(55) && diff <= Duration::minutes(65);
        if !(is_24h || is_1h) {
            continue;
        }
        let reminder_type = if is_24h { "24h" } else { "1h" };
        let locale = queries::get_user_locale(pool, appt.client_id, appt.user_id)
            .await
            .ok()
            .flatten()
            .map(|value| normalize_locale(&value).to_string())
            .unwrap_or_else(|| default_locale.clone());
        let msg = format_reminder_message(&appt, &locale);
        let _ = bot
            .send_message(teloxide::types::ChatId(appt.user_id), msg)
            .reply_markup(reminder_keyboard(&locale))
            .await;
        info!(
            "Reminder sent: Appointment ID {}, User ID {}, Type: {}",
            appt.id, appt.user_id, reminder_type
        );
    }
    Ok(())
}

pub fn format_reminder_message(appointment: &Appointment, locale: &str) -> String {
    let mut args = HashMap::new();
    args.insert("date", appointment.appointment_date.to_string());
    args.insert("time", appointment.appointment_time.format("%H:%M").to_string());
    args.insert("service_type", appointment.service_type.clone());
    tr_with_args(locale, "reminder-message", &args)
}
