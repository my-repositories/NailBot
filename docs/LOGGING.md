# Logging Guide

## Levels

* `TRACE` — Detailed flow (e.g., FSM state change).
* `DEBUG` — Internal operations (DB query, scheduler task).
* `INFO` — User actions (booking, admin commands).
* `WARN` — Recoverable issues (network retry, invalid input).
* `ERROR` — Critical failures (DB unavailable, API error).

## Format

* Use structured logging (`tracing`).
* Include context: `user_id`, `appointment_id`, `state`.
* Timestamp in UTC.

## Examples

* **INFO:**
  ```
  User 12345 started booking flow
  Appointment 42 created for user 12345
  Admin 999 canceled appointment 42
  ```
* **WARN:**
  ```
  Failed to send reminder for appointment 42 (user 12345): Network error. Retrying...
  ```
* **ERROR:**
  ```
  Database connection failed: SQLite error: unable to open database file
  ```

## Setup

* Add to `Cargo.toml`:
  ```toml
  [dependencies]
  tracing = "0.1"
  tracing-subscriber = "0.3"
  ```
* Initialize in `main.rs`:
  ```rust
  tracing_subscriber::fmt::init();
  ```

## AI Prompts

> Add logging to `handle_appointment` function. Log at `INFO`: «User {user_id} started booking». Log at `DEBUG`: «Selected date: {date}, time: {time}». Use `tracing::info!` and `tracing::debug!`.

> Implement error logging in `schedule_reminder`. On failure, log `ERROR`: «Failed to schedule reminder for appointment {id}: {error}». Include `user_id` in context.
