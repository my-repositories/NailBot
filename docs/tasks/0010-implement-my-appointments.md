# Task 0010: Implement «My Appointments» Functionality

## Goal
Enable users to view, manage, and modify their existing appointments through the bot interface.

## Context
Tenth task in the MVP phase (`ROADMAP.md`). Follows reminder implementation. Provides users with visibility into their booking history and control over their appointments.


## Requirements
* [ ] Implement handler for `/myappointments` command in `src/handlers/user.rs`
* [ ] Display list of user’s appointments (upcoming and recent past)
* [ ] For each appointment, show: date, time, status (confirmed/cancelled), service type
* [ ] Add inline buttons for each upcoming appointment: «Cancel», «Reschedule»
* [ ] Paginate results if more than 5 appointments exist
* [ ] Include «Back to Menu» button
* [ ] Update appointment status in DB when user cancels
* [ ] Preserve data consistency: cancelled appointments shouldn’t trigger reminders

## Technical Details
* Files:
  * `src/handlers/user.rs` (extend with my appointments handlers)
  * `src/database/queries.rs` (add `get_user_appointments()` function)
  * `src/keyboards/my_appointments.rs` (new — my appointments keyboard)
  * `src/keyboards/mod.rs` (update to include my_appointments module)
* Documentation: `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, `docs/UI_GUIDE.md`
* Libraries: `teloxide`, `rusqlite`, `chrono`
* Key considerations:
  * Query DB by `user_id` and sort by date (upcoming first)
  * Tenant isolation (SaaS): all queries are scoped to `client_id`; callbacks must verify appointment belongs to the current tenant
  * Limit displayed appointments to last 3 months
  * Handle timezone conversion: display times in user’s local time
  * Ensure cancellation updates both appointment status and reminder schedule
  * Use inline keyboards for actions (not reply keyboards)

## Acceptance Criteria
* On `/myappointments`:
  * Bot sends message: «👤 Your Appointments:»
  * Lists all relevant appointments in chronological order
* Each appointment displays:
  ```
  🗓️ {date} at {time}
  👤 Service: {service_type}
  ✅ Status: {status}
  ```
* For upcoming appointments (status = confirmed):
  * Inline buttons «❌ Cancel» and «🗓️ Reschedule» appear
  * Buttons include appointment ID in callback data
* For past/cancelled appointments:
  * No action buttons shown
  * Status is clearly marked
* Pagination:
  * If > 5 appointments, show «Next Page » button
  * «Previous Page » appears on page 2+
  * Current page indicator: «Page 1/3»
* «Back to Menu»:
  * Returns to main menu (`Idle` state)
  * Uses `keyboards::mainmenu::mainmenu()` keyboard
* Cancellation:
  * Updates DB: `status = 'cancelled'`
  * Cancels any pending reminders for this appointment
  * Sends confirmation: «✅ Appointment on {date} at {time} has been cancelled.»
  * Refreshes the appointments list
* SaaS tenant isolation: user in tenant A cannot list/cancel/reschedule appointments belonging to tenant B (even if they guess IDs)

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, and `docs/UI_GUIDE.md`, implement task 0010.

1. Update `src/keyboards/mod.rs` to include the my appointments module:
   ```rust
   // File: src/keyboards/mod.rs
   pub mod main_menu;
   pub mod calendar;
   pub mod time_slots;
   pub mod confirmation;
   pub mod reminder;
   pub mod my_appointments;

   // Re‑export for convenience
   pub use main_menu::main_menu;
   pub use calendar::calendar_keyboard;
   pub use time_slots::time_slots_keyboard;
   pub use confirmation::confirmation_keyboard;
   pub use reminder::reminder_keyboard;
   pub use my_appointments::my_appointments_keyboard;
   ```
2. Create `src/keyboards/my_appointments.rs`:
   ```rust
   // File: src/keyboards/my_appointments.rs
   use teloxide::types::{InlineKeyboardMarkup, InlineKeyboardButton};

   pub fn my_appointments_keyboard(appointments: &[Appointment], page: usize, total_pages: usize) -> InlineKeyboardMarkup {
       let mut rows = vec![];

       // Add buttons for each appointment (Cancel/Reschedule)
       // Add pagination buttons if needed
       // Add «Back to Menu» button

       InlineKeyboardMarkup::new(rows)
   }
   ```
3. In `src/database/queries.rs`, implement `get_user_appointments(user_id: i64, page: usize) -> Result<Vec<Appointment>, Box<dyn std::error::Error>>`:
   * Query `appointments` table by `user_id`
   * Filter by last 3 months
   * Sort: upcoming first, then recent past
   * Paginate: limit 5 results per page
4. In `src/handlers/user.rs`, handle `/myappointments` and callbacks:
   * `/myappointments` handler:
     * Call `get_user_appointments(user_id, 1)`
     * Format list message
     * Attach `my_appointments_keyboard()`
   * `cancel_appointment_from_list` callback:
     * Update DB status to `cancelled`
     * Cancel reminders
     * Send confirmation
     * Refresh list
   * `reschedule_from_list` callback:
     * Start new booking flow
     * Pre‑fill data if possible
   * Pagination callbacks (`next_page`, `prev_page`):
     * Fetch next/previous page
     * Edit message with new list and keyboard
5. Ensure all actions are logged via `tracing::info!`.
