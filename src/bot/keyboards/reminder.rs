use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::shared::i18n::tr;

pub fn reminder_keyboard(locale: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            tr(locale, "reminder-cancel-button"),
            "cancel_appointment_from_reminder",
        )],
        vec![InlineKeyboardButton::callback(
            tr(locale, "reminder-reschedule-button"),
            "reschedule_from_reminder",
        )],
    ])
}
