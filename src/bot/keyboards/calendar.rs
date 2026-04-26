use chrono::{Datelike, NaiveDate, Utc};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn calendar_keyboard(year: i32, month: u32) -> InlineKeyboardMarkup {
    let Some(first) = NaiveDate::from_ymd_opt(year, month, 1) else {
        return InlineKeyboardMarkup::default();
    };
    let mut rows: Vec<Vec<InlineKeyboardButton>> = vec![vec![
        InlineKeyboardButton::callback("Mo", "noop"),
        InlineKeyboardButton::callback("Tu", "noop"),
        InlineKeyboardButton::callback("We", "noop"),
        InlineKeyboardButton::callback("Th", "noop"),
        InlineKeyboardButton::callback("Fr", "noop"),
        InlineKeyboardButton::callback("Sa", "noop"),
        InlineKeyboardButton::callback("Su", "noop"),
    ]];

    let today = Utc::now().date_naive();
    let mut current_row = Vec::new();
    for _ in 1..first.weekday().number_from_monday() {
        current_row.push(InlineKeyboardButton::callback(" ", "noop"));
    }

    let mut day = 1;
    while let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
        let label = if date == today {
            format!("🟢{}", day)
        } else {
            day.to_string()
        };
        current_row.push(InlineKeyboardButton::callback(
            label,
            format!("date_select:{date}"),
        ));
        if current_row.len() == 7 {
            rows.push(current_row);
            current_row = Vec::new();
        }
        day += 1;
    }
    if !current_row.is_empty() {
        rows.push(current_row);
    }

    rows.push(vec![
        InlineKeyboardButton::callback("⬅️ Previous Month", format!("nav_prev:{year}-{month:02}")),
        InlineKeyboardButton::callback("Next Month ➡️", format!("nav_next:{year}-{month:02}")),
    ]);
    rows.push(vec![InlineKeyboardButton::callback("🔙 Back to Menu", "back_to_menu")]);

    InlineKeyboardMarkup::new(rows)
}
