use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Appointment {
    pub id: i64,
    pub client_id: i64,
    pub user_id: i64,
    pub appointment_date: NaiveDate,
    pub appointment_time: NaiveTime,
    pub service_type: String,
    pub status: String,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppointmentDraft {
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub service_type: Option<String>,
}
