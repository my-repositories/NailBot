use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn confirmation_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("✅ Confirm", "confirm_appointment")],
        vec![InlineKeyboardButton::callback("❌ Cancel", "cancel_appointment")],
        vec![InlineKeyboardButton::callback("🔙 Back to Edit", "back_to_edit")],
    ])
}
