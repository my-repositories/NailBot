use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::shared::error::AppError;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode {
    Saas,
    OnPrem,
}

impl FromStr for Mode {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "saas" => Ok(Self::Saas),
            "onprem" => Ok(Self::OnPrem),
            _ => Err(AppError::Config(format!("invalid MODE: {s}"))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub mode: Mode,
    pub database_url: String,
    pub bot_token: String,
    pub default_locale: String,
    pub admin_ids: Vec<i64>,
    pub license_key: Option<String>,
}

impl Settings {
    pub fn from_env() -> Result<Self, AppError> {
        let mode = std::env::var("MODE")
            .unwrap_or_else(|_| "onprem".to_string())
            .parse()?;
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/nailbot".to_string());
        let bot_token = std::env::var("BOT_TOKEN").unwrap_or_default();
        let default_locale = std::env::var("DEFAULT_LOCALE").unwrap_or_else(|_| "en".to_string());
        let admin_ids = std::env::var("ADMIN_IDS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|v| v.trim().parse::<i64>().ok())
            .collect::<Vec<_>>();
        let license_key = std::env::var("LICENSE_KEY").ok();

        Ok(Self {
            mode,
            database_url,
            bot_token,
            default_locale,
            admin_ids,
            license_key,
        })
    }
}
