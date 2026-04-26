use teloxide::types::{KeyboardButton, KeyboardMarkup};

use crate::shared::i18n::tr;

pub fn main_menu(locale: &str) -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new(tr(locale, "menu-book-appointment"))],
        vec![
            KeyboardButton::new(tr(locale, "menu-my-appointments")),
            KeyboardButton::new(tr(locale, "menu-prices")),
        ],
        vec![KeyboardButton::new(tr(locale, "menu-portfolio"))],
    ])
    .resize_keyboard()
}
