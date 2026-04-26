# Task 0008: Implement Appointment Confirmation

## Goal
Enable the user to review and confirm their appointment details before finalizing the booking. Integrate with the modular keyboard system (`src/keyboards/`).

## Context
Eighth task in the MVP phase (`ROADMAP.md`). Follows user data collection. Completes the core booking flow by allowing users to verify their choices and commit to the appointment.

## API/Bot Split Alignment
* [ ] Bot renders confirmation summary and receives user decision.
* [ ] Appointment creation/commit is executed by API endpoint(s), not by bot-side repository calls.
* [ ] API is source of truth for idempotency, conflict handling, and persistence transactions.

## Requirements
* [ ] Implement handler for `Confirming` state in `src/handlers/user.rs`
* [ ] Display a summary of the appointment: date, time, name, phone
* [ ] Use `src/keyboards/confirmation.rs` for confirmation keyboard
* [ ] Add two buttons: «✅ Confirm» and «❌ Cancel»
* [ ] On confirmation:
  * [ ] Save the appointment to the database
  * [ ] Advance FSM to `Idle` state
  * [ ] Send a confirmation message with details
* [ ] On cancellation:
  * [ ] Reset FSM to `Idle`
  * [ ] Send cancellation message and return to main menu
* [ ] Allow user to go back and edit any detail

## Technical Details
* Files:
  * `src/handlers/user.rs` (extend with confirmation handlers)
  * `src/database/queries.rs` (add `save_appointment()` function)
  * `src/keyboards/confirmation.rs` (new — confirmation keyboard)
  * `src/keyboards/mod.rs` (update to include confirmation module)
* Documentation: `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, `docs/UI_GUIDE.md`, `docs/FSM_GUIDE.md`
* Libraries: `teloxide`, `sqlx`, `chrono`
* Key considerations:
  * Format the confirmation message clearly and readably
  * Ensure all data from `UserSession.appointment_data` is included
  * Use transactional writes to avoid partial saves
  * Log confirmation/cancellation events
  * Redact PII in logs (e.g., phone numbers)
  * Tenant isolation (SaaS): appointment inserts and reads are scoped to `client_id`; callbacks must not allow cross-tenant appointment manipulation

## Acceptance Criteria
* In `Confirming` state:
  * Bot sends a message with formatted summary:
    ```
    🗓️ Date: {date}
    ⏰ Time: {time}
    👤 Name: {name}
    📱 Phone: {phone}

    Please confirm your appointment:
    ```
  * Inline keyboard shows «✅ Confirm» and «❌ Cancel» buttons
* On «Confirm»:
  * Appointment is saved to `appointments` table
  * User receives: «✅ Appointment confirmed! See you on {date} at {time}.»
  * FSM state resets to `Idle`
* On «Cancel»:
  * User receives: «❌ Appointment cancelled. Use /appointment to book again.»
  * FSM resets to `Idle`, main menu is shown
* Optional «Back to Edit» button:
  * Allows user to return to any previous step (e.g., change date or time)
  * Preserves already entered data

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, `docs/UI_GUIDE.md`, and `docs/FSM_GUIDE.md`, implement task 0008.

1. Update `src/keyboards/mod.rs` to include the confirmation module:
   ```rust
   // File: src/keyboards/mod.rs
   pub mod main_menu;
   pub mod calendar;
   pub mod time_slots;
   pub mod confirmation;

   // Re‑export for convenience
   pub use main_menu::main_menu;
   pub use calendar::calendar_keyboard;
   pub use time_slots::time_slots_keyboard;
   pub use confirmation::confirmation_keyboard;
   ```
2. Create `src/keyboards/confirmation.rs`:
   ```rust
   // File: src/keyboards/confirmation.rs
   use teloxide::types::InlineKeyboardMarkup;
   use teloxide::types::InlineKeyboardButton;

   pub fn confirmation_keyboard() -> InlineKeyboardMarkup {
       InlineKeyboardMarkup::new(vec![
           vec![InlineKeyboardButton::callback("✅ Confirm", "confirm_appointment".to_string())],
           vec![InlineKeyboardButton::callback("❌ Cancel", "cancel_appointment".to_string())]
           // Optionally add:
           // vec![InlineKeyboardButton::callback("🔙 Back to Edit", "back_to_edit".to_string())]
       ])
   }
   ```
3. In `src/database/queries.rs`, implement `save_appointment(appointment: &Appointment) -> Result<i64, Box<dyn std::error::Error>>`:
   * Insert into `appointments` table using fields from `Appointment` struct
   * Return the new appointment ID
4. In `src/handlers/user.rs`, handle `Confirming` state:
   * On entering state:
     * Format and send confirmation message with all collected data
     * Attach `confirmation_keyboard()`
   * Handle `confirm_appointment` callback:
     * Build `Appointment` from `UserSession.appointment_data`
     * Call `save_appointment()`
     * On success:
       * Send confirmation message
       * Reset state to `Idle`
       * Log event: `tracing::info!("Appointment confirmed: ID {}", appointment_id)`
     * On error: send error message, stay in `Confirming`
   * Handle `cancel_appointment`:
     * Send cancellation message
     * Reset state to `Idle`, show main menu
     * Log cancellation: `tracing::info!("Appointment cancelled: User ID {}", user_id)`
   * Optionally handle `back_to_edit`:
     * Return to `SelectingDate`, preserving partial data
     * Start booking flow again
5. Ensure all state transitions and user actions are logged via `tracing::info!`, with phone numbers redacted in logs.
