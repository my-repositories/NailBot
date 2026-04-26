use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppointmentDraft {
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub service_type: Option<String>,
}
