pub mod admin;
pub mod user;

use serde::Deserialize;
use teloxide::types::UserId;

use crate::shared::i18n::normalize_locale;

#[derive(Debug, Deserialize)]
struct UserLocaleResponse {
    locale: String,
}

pub async fn resolve_user_locale(
    api_base_url: &str,
    client_id: i64,
    user_id: UserId,
    default_locale: &str,
) -> String {
    let fallback = normalize_locale(default_locale).to_string();
    let url = format!(
        "{}/api/v1/users/{}/locale?client_id={}",
        api_base_url.trim_end_matches('/'),
        user_id.0,
        client_id
    );
    match reqwest::get(&url).await {
        Ok(resp) if resp.status().is_success() => resp
            .json::<UserLocaleResponse>()
            .await
            .map(|data| normalize_locale(&data.locale).to_string())
            .unwrap_or(fallback),
        _ => fallback,
    }
}
