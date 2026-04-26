use teloxide::types::KeyboardMarkup;

pub fn main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec!["🗓️ Book Appointment".to_string()],
        vec!["👤 My Appointments".to_string(), "💰 Prices".to_string()],
        vec!["📸 Portfolio".to_string()],
    ])
    .resize_keyboard()
}
