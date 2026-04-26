use std::collections::HashMap;

use chrono::{DateTime, Utc};
use teloxide::types::UserId;
use tokio::sync::RwLock;

use crate::database::models::AppointmentDraft;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppointmentState {
    Idle,
    SelectingDate,
    SelectingTime,
    InputtingName,
    InputtingPhone,
    Confirming,
}

#[derive(Debug, Clone)]
pub struct UserSession {
    pub state: AppointmentState,
    pub appointment_data: AppointmentDraft,
    pub last_activity: DateTime<Utc>,
}

pub type SessionKey = (i64, UserId);
pub type SessionStore = RwLock<HashMap<SessionKey, UserSession>>;

impl UserSession {
    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        (now - self.last_activity).num_minutes() >= 10
    }
}
