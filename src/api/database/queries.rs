use chrono::{Duration, NaiveDate, NaiveTime, Utc};
use sqlx::PgPool;

use super::models::Appointment;

pub async fn get_available_time_slots(
    pool: &PgPool,
    client_id: i64,
    date: NaiveDate,
) -> Result<Vec<String>, sqlx::Error> {
    let booked: Vec<NaiveTime> = sqlx::query_scalar(
        "SELECT appointment_time FROM appointments WHERE client_id = $1 AND appointment_date = $2 AND status = 'confirmed'",
    )
    .bind(client_id)
    .bind(date)
    .fetch_all(pool)
    .await?;

    let mut slots = Vec::new();
    let mut t = NaiveTime::from_hms_opt(9, 0, 0).expect("valid constant time");
    let end = NaiveTime::from_hms_opt(20, 0, 0).expect("valid constant time");
    while t < end {
        if !booked.contains(&t) {
            slots.push(t.format("%H:%M").to_string());
        }
        t += Duration::minutes(30);
    }
    Ok(slots)
}

pub async fn save_appointment(
    pool: &PgPool,
    appointment: &Appointment,
) -> Result<i64, sqlx::Error> {
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO appointments (client_id, user_id, appointment_date, appointment_time, service_type, status)
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
    )
    .bind(appointment.client_id)
    .bind(appointment.user_id)
    .bind(appointment.appointment_date)
    .bind(appointment.appointment_time)
    .bind(&appointment.service_type)
    .bind(&appointment.status)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

pub async fn get_user_appointments(
    pool: &PgPool,
    client_id: i64,
    user_id: i64,
    page: usize,
) -> Result<Vec<Appointment>, sqlx::Error> {
    let offset = ((page.saturating_sub(1)) * 5) as i64;
    let three_months_ago = Utc::now().date_naive() - Duration::days(90);
    sqlx::query_as::<_, Appointment>(
        r#"
        SELECT id, client_id, user_id, appointment_date, appointment_time, service_type, status, created_at
        FROM appointments
        WHERE client_id = $1 AND user_id = $2 AND appointment_date >= $3
        ORDER BY appointment_date ASC, appointment_time ASC
        LIMIT 5 OFFSET $4
        "#,
    )
    .bind(client_id)
    .bind(user_id)
    .bind(three_months_ago)
    .bind(offset)
    .fetch_all(pool)
    .await
}
