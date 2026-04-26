use chrono::NaiveDate;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn time_slots_keyboard(date: NaiveDate, slots: &[String]) -> InlineKeyboardMarkup {
    let mut rows = slots
        .iter()
        .map(|slot| {
            vec![InlineKeyboardButton::callback(
                slot.clone(),
                format!("time_select:{date} {slot}"),
            )]
        })
        .collect::<Vec<_>>();
    rows.push(vec![InlineKeyboardButton::callback(
        "🔙 Back",
        "back_to_calendar",
    )]);
    rows.push(vec![InlineKeyboardButton::callback(
        "🗓️ Reschedule Date",
        "reschedule_date",
    )]);
    InlineKeyboardMarkup::new(rows)
}
