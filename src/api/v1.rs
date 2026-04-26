use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use chrono::{NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::api::database::{models::Appointment, queries};

#[derive(Clone)]
pub struct ApiState {
    pub pool: PgPool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct AvailabilityQuery {
    pub client_id: i64,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAppointmentRequest {
    pub client_id: i64,
    pub user_id: i64,
    pub appointment_date: String,
    pub appointment_time: String,
    pub service_type: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAppointmentResponse {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ListAppointmentsQuery {
    pub client_id: i64,
    pub user_id: i64,
    pub page: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct TodayAppointmentsQuery {
    pub client_id: i64,
}

#[derive(Debug, Serialize)]
pub struct UserLocaleResponse {
    pub locale: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = ApiState { pool };
    Router::new()
        .route("/health", get(health))
        .route("/availability", get(availability))
        .route("/appointments", post(create_appointment).get(list_appointments))
        .route("/appointments/:id", delete(cancel_appointment))
        .route("/users/:telegram_id/locale", get(user_locale))
        .route("/admin/appointments/today", get(today_appointments))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn availability(
    State(state): State<ApiState>,
    Query(query): Query<AvailabilityQuery>,
) -> Result<Json<Vec<String>>, (StatusCode, Json<ErrorResponse>)> {
    let date = NaiveDate::parse_from_str(&query.date, "%Y-%m-%d").map_err(bad_request)?;
    let slots = queries::get_available_time_slots(&state.pool, query.client_id, date)
        .await
        .map_err(internal_error)?;
    Ok(Json(slots))
}

async fn create_appointment(
    State(state): State<ApiState>,
    Json(request): Json<CreateAppointmentRequest>,
) -> Result<(StatusCode, Json<CreateAppointmentResponse>), (StatusCode, Json<ErrorResponse>)> {
    let date = NaiveDate::parse_from_str(&request.appointment_date, "%Y-%m-%d").map_err(bad_request)?;
    let time = NaiveTime::parse_from_str(&request.appointment_time, "%H:%M").map_err(bad_request)?;

    let appointment = Appointment {
        id: 0,
        client_id: request.client_id,
        user_id: request.user_id,
        appointment_date: date,
        appointment_time: time,
        service_type: request.service_type,
        status: "confirmed".to_string(),
        created_at: Utc::now(),
    };
    let id = queries::save_appointment(&state.pool, &appointment)
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(CreateAppointmentResponse { id })))
}

async fn list_appointments(
    State(state): State<ApiState>,
    Query(query): Query<ListAppointmentsQuery>,
) -> Result<Json<Vec<Appointment>>, (StatusCode, Json<ErrorResponse>)> {
    let page = query.page.unwrap_or(1);
    let appointments = queries::get_user_appointments(&state.pool, query.client_id, query.user_id, page)
        .await
        .map_err(internal_error)?;
    Ok(Json(appointments))
}

async fn cancel_appointment(
    State(state): State<ApiState>,
    Path(id): Path<i64>,
    Query(query): Query<TodayAppointmentsQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    sqlx::query("UPDATE appointments SET status = 'cancelled' WHERE id = $1 AND client_id = $2")
        .bind(id)
        .bind(query.client_id)
        .execute(&state.pool)
        .await
        .map_err(internal_error)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn today_appointments(
    State(state): State<ApiState>,
    Query(query): Query<TodayAppointmentsQuery>,
) -> Result<Json<Vec<Appointment>>, (StatusCode, Json<ErrorResponse>)> {
    let today = Utc::now().date_naive();
    let appointments = sqlx::query_as::<_, Appointment>(
        "SELECT id, client_id, user_id, appointment_date, appointment_time, service_type, status, created_at
         FROM appointments
         WHERE client_id = $1 AND appointment_date = $2
         ORDER BY appointment_time ASC",
    )
    .bind(query.client_id)
    .bind(today)
    .fetch_all(&state.pool)
    .await
    .map_err(internal_error)?;
    Ok(Json(appointments))
}

async fn user_locale(
    State(state): State<ApiState>,
    Path(telegram_id): Path<i64>,
    Query(query): Query<TodayAppointmentsQuery>,
) -> Result<Json<UserLocaleResponse>, (StatusCode, Json<ErrorResponse>)> {
    let locale = queries::get_user_locale(&state.pool, query.client_id, telegram_id)
        .await
        .map_err(internal_error)?
        .unwrap_or_else(|| "en".to_string());
    Ok(Json(UserLocaleResponse { locale }))
}

fn bad_request(err: impl std::fmt::Display) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            code: "bad_request".to_string(),
            message: err.to_string(),
        }),
    )
}

fn internal_error(err: impl std::fmt::Display) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            code: "internal_error".to_string(),
            message: err.to_string(),
        }),
    )
}
