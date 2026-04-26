use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn reminder_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            "❌ Cancel Appointment",
            "cancel_appointment_from_reminder",
        )],
        vec![InlineKeyboardButton::callback(
            "🗓️ Reschedule",
            "reschedule_from_reminder",
        )],
    ])
}
