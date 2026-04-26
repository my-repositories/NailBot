# Task 0009: Implement Appointment Reminders

## Goal
Implement automated reminders sent to users 24 hours and 1 hour before their appointment. Integrate with the modular keyboard system (`src/keyboards/`).


## Context
Ninth task in the MVP phase (`ROADMAP.md`). Follows appointment confirmation. Reduces no‑shows by proactively reminding users of their upcoming appointments.

## API/Bot Split Alignment
* [ ] Reminder eligibility/scans should be orchestrated by backend scheduler/API service.
* [ ] Bot adapter is responsible for message delivery and callback transport.
* [ ] In split deployments, reminder job ownership must avoid duplicate sends across workers.


## Requirements
* [ ] Create a background job that checks for upcoming appointments
* [ ] Send reminder 24 hours before appointment
* [ ] Send reminder 1 hour before appointment
* [ ] Include appointment details and option to cancel/reschedule
* [ ] Use `src/keyboards/reminder.rs` for reminder keyboard
* [ ] Log reminder delivery status
* [ ] Ensure reminders don’t go to cancelled appointments

## Technical Details
* Files:
  * `src/reminders.rs` (new — for reminder logic)
  * `src/main.rs` (add job scheduler)
  * `src/handlers/user.rs` (extend with cancel/reschedule handlers)
  * `src/keyboards/reminder.rs` (new — reminder keyboard)
  * `src/keyboards/mod.rs` (update to include reminder module)
* Documentation: `docs/REMINDERS.md`, `docs/BUSINESS_RULES.md`, `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`
* Libraries: `teloxide`, `tokio`, `chrono`, `tokio‑cron‑scheduler` (or `tokio::time` for basic scheduling)
* Key considerations:
  * Use UTC for all datetime comparisons
  * Query DB for appointments with `status = 'confirmed'` and appropriate time windows
  * Handle timezone conversion: bot runs in UTC+3, but user times are stored in local time
  * Rate‑limit messages to avoid spamming
  * Ensure callback handlers are idempotent (safe to retry)
  * Tenant isolation (SaaS): reminder scans and send operations must be scoped to `client_id` and use the correct tenant bot token
  * SaaS scaling: if multiple workers run, use a DB-coordinated claim model (see `docs/REMINDERS.md`)

## Acceptance Criteria
* Reminders are sent:
  * Exactly 24 hours before the appointment
  * Exactly 1 hour before the appointment
* Reminder message includes:
  ```
  ⏰ Reminder: Your appointment is tomorrow at {time}!

  🗓️ Date: {date}
  ⏰ Time: {time}
  👤 Name: {name}

  To cancel or reschedule, use /appointment
  ```
* Inline keyboard includes:
  * «❌ Cancel Appointment» (callback: `cancel_appointment_from_reminder`)
  * «🗓️ Reschedule» (callback: `reschedule_from_reminder`)
* Logs show:
  * «Reminder sent: Appointment ID {id}, User ID {user_id}, Type: 24h»
  * Delivery failures (e.g., blocked bot) are logged as errors
* No reminders are sent for:
  * Cancelled appointments
  * Appointments within the next 5 minutes (too late)
  * Already‑reminded appointments (prevent duplicates)

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md`, `docs/REMINDERS.md` and `docs/BUSINESS_RULES.md`, implement task 0009.

1. Update `src/keyboards/mod.rs` to include the reminder module:
   ```rust
   // File: src/keyboards/mod.rs
   pub mod main_menu;
   pub mod calendar;
   pub mod time_slots;
   pub mod confirmation;
   pub mod reminder;

   // Re‑export for convenience
   pub use main_menu::main_menu;
   pub use calendar::calendar_keyboard;
   pub use time_slots::time_slots_keyboard;
   pub use confirmation::confirmation_keyboard;
   pub use reminder::reminder_keyboard;
   ```
2. Create `src/keyboards/reminder.rs`:
   ```rust
   // File: src/keyboards/reminder.rs
   use teloxide::types::{InlineKeyboardMarkup, InlineKeyboardButton};

   pub fn reminder_keyboard() -> InlineKeyboardMarkup {
       InlineKeyboardMarkup::new(vec![
           vec![InlineKeyboardButton::callback("❌ Cancel Appointment", "cancel_appointment_from_reminder".to_string())],
           vec![InlineKeyboardButton::callback("🗓️ Reschedule", "reschedule_from_reminder".to_string())]
       ])
   }
   ```
3. In `src/reminders.rs`, create a module with:
   * Function `start_reminder_service()` — spawns a tokio task that runs every 30 minutes
   * Function `check_and_send_reminders(bot: &Bot)` — queries DB for appointments needing reminders
   * Helper `format_reminder_message(appointment: &Appointment) -> String`
4. Implement logic in `check_and_send_reminders()`:
   * Query DB for confirmed appointments where:
     * `(appointment_time - now) BETWEEN 23h55m AND 24h05m` → 24h reminder
     * `(appointment_time - now) BETWEEN 55m AND 65m` → 1h reminder
   * For each match:
     * Send message via `bot.send_message()`
     * Attach `reminder_keyboard()`
     * Log success/failure
5. In `src/handlers/user.rs`, handle callbacks:
   * `cancel_appointment_from_reminder`:
     * Update DB: set `status = 'cancelled'`
     * Send confirmation: «✅ Your appointment on {date} at {time} has been cancelled.»
     * Reset any pending reminders
   * `reschedule_from_reminder`:
     * Start new booking flow (`/appointment`)
     * Preserve user context if possible
6. In `src/main.rs`, call `start_reminder_service(bot.clone())` during startup.
7. Ensure all reminder operations are wrapped in transactions where appropriate.

