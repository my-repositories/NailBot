use crate::database::models::Appointment;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn admin_dashboard_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("📊 Today's Stats", "stats_today")],
        vec![InlineKeyboardButton::callback(
            "🗓️ Today's Appointments",
            "today_appointments",
        )],
        vec![InlineKeyboardButton::callback("📈 Weekly Overview", "weekly_stats")],
        vec![InlineKeyboardButton::callback("⚙️ Settings", "admin_settings")],
    ])
}

pub fn appointments_list_keyboard(appointments: &[Appointment]) -> InlineKeyboardMarkup {
    let mut rows = Vec::new();
    for a in appointments {
        rows.push(vec![InlineKeyboardButton::callback(
            format!("✅ {} at {}", a.service_type, a.appointment_time.format("%H:%M")),
            format!("mark_done:{}", a.id),
        )]);
    }
    rows.push(vec![InlineKeyboardButton::callback("🔙 Back", "back_to_admin")]);
    InlineKeyboardMarkup::new(rows)
}
