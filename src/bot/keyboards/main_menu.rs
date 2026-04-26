use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new("🗓️ Book Appointment")],
        vec![
            KeyboardButton::new("👤 My Appointments"),
            KeyboardButton::new("💰 Prices"),
        ],
        vec![KeyboardButton::new("📸 Portfolio")],
    ])
    .resize_keyboard()
}
