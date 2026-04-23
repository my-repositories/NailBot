# Scheduler Guide

Hybrid requirement:

- **SaaS (multi-tenant)**: scheduler state must be tenant-aware and must send messages using the correct tenant bot identity.
- **On‑Prem (single-tenant)**: scheduler is simpler; only one tenant and one bot token.

## Components

* **Scheduler Service** (`src/scheduler/reminders.rs`): Manages reminder tasks.
* **Task Storage:**
  - On‑Prem: `HashMap<i64, tokio::task::JoinHandle<()>>` (appointment_id → task) is acceptable for MVP.
  - SaaS: prefer `HashMap<(i64, i64), tokio::task::JoinHandle<()>>` ((client_id, appointment_id) → task) to avoid collisions and enforce isolation.
* **Recovery:** Load active appointments on startup.

## Reminder Logic

1. On booking: Calculate reminder_time = appointment_time - 24 hours.
2. Schedule task via `tokio::spawn`.
3. Task sends message: «Reminder: Your appointment is tomorrow at <time>».
4. On cancellation: Remove task from storage.
5. On startup: Load appointments from DB → Schedule reminders.

## Functions

* `schedule_reminder(appointment_id: i64, user_id: i64, reminder_time: DateTime<Utc>)`
* `cancel_reminder(appointment_id: i64)`
* `recover_reminders()`

## AI Prompts


> Implement `schedule_reminder` function in `src/scheduler/reminders.rs`. Use `tokio::spawn` to create async task. Send reminder message 24 hours before appointment. Store task handle in `HashMap`. Log success/failure.

> Write `recover_reminders` function. Query DB for future appointments. Call `schedule_reminder` for each. Log recovered count.

## SaaS scaling note

If you run more than one worker, use the DB-coordinated model described in `docs/REMINDERS.md` to avoid duplicate reminders.