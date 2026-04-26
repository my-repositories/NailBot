use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
pub use crate::shared::models::{Appointment, AppointmentDraft};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub client_id: i64,
    pub telegram_id: i64,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub locale: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub id: i64,
    pub client_id: i64,
    pub slot_date: NaiveDate,
    pub slot_time: NaiveTime,
    pub is_available: bool,
}

