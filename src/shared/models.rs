use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl<'r> FromRow<'r, PgRow> for Appointment {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            client_id: row.try_get("client_id")?,
            user_id: row.try_get("user_id")?,
            appointment_date: row.try_get("appointment_date")?,
            appointment_time: row.try_get("appointment_time")?,
            service_type: row.try_get("service_type")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppointmentDraft {
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub service_type: Option<String>,
}
