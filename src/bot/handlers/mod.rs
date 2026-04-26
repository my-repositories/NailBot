pub mod admin;
pub mod user;

use sqlx::PgPool;
use teloxide::types::UserId;

use crate::api::database::queries;
use crate::shared::i18n::normalize_locale;

pub async fn resolve_user_locale(
    pool: &PgPool,
    client_id: i64,
    user_id: UserId,
    default_locale: &str,
) -> String {
    let fallback = normalize_locale(default_locale).to_string();
    queries::get_user_locale(pool, client_id, user_id.0 as i64)
        .await
        .ok()
        .flatten()
        .map(|value| normalize_locale(&value).to_string())
        .unwrap_or(fallback)
}
