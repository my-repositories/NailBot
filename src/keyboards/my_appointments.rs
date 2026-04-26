use crate::database::models::Appointment;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn my_appointments_keyboard(
    appointments: &[Appointment],
    page: usize,
    total_pages: usize,
) -> InlineKeyboardMarkup {
    let mut rows = Vec::new();
    for appt in appointments {
        if appt.status == "confirmed" {
            rows.push(vec![
                InlineKeyboardButton::callback(
                    format!("❌ Cancel #{}", appt.id),
                    format!("cancel_appointment_from_list:{}", appt.id),
                ),
                InlineKeyboardButton::callback(
                    format!("🗓️ Reschedule #{}", appt.id),
                    format!("reschedule_from_list:{}", appt.id),
                ),
            ]);
        }
    }
    if page > 1 {
        rows.push(vec![InlineKeyboardButton::callback("⬅️ Previous", format!("prev_page:{page}"))]);
    }
    if page < total_pages {
        rows.push(vec![InlineKeyboardButton::callback("Next ➡️", format!("next_page:{page}"))]);
    }
    rows.push(vec![InlineKeyboardButton::callback(
        format!("Page {page}/{total_pages}"),
        "noop",
    )]);
    rows.push(vec![InlineKeyboardButton::callback("🔙 Back to Menu", "back_to_menu")]);
    InlineKeyboardMarkup::new(rows)
}
