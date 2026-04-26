use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::shared::i18n::tr;

pub fn confirmation_keyboard(locale: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            tr(locale, "confirm-button"),
            "confirm_appointment",
        )],
        vec![InlineKeyboardButton::callback(
            tr(locale, "cancel-button"),
            "cancel_appointment",
        )],
        vec![InlineKeyboardButton::callback(
            tr(locale, "back-to-edit-button"),
            "back_to_edit",
        )],
    ])
}
