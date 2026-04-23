# Task 0004: Add FSM for Booking Flow

## Goal
Implement a finite‑state machine (FSM) to manage conversation states during the appointment booking process.

## Context
Fourth task in the MVP phase (`ROADMAP.md`). Follows the implementation of basic handlers. Enables guiding the user through a multi‑step booking process.

## Requirements
* [ ] Define an enum `AppointmentState` with all states from `docs/FSM_GUIDE.md`.
* [ ] Implement state transitions: `Idle` → `SelectingDate` → `SelectingTime` → `InputtingName` → `InputtingPhone` → `Confirming`.
* [ ] Add a state timeout (10 minutes) with return to `Idle`.
* [ ] Log state changes (`docs/LOGGING.md`).
* [ ] Store user state in memory (use `HashMap<UserId, AppointmentState>` for MVP).
* [ ] Hybrid session keying:
  * [ ] SaaS: store session by `(client_id, user_id)` to prevent cross-tenant bleed
  * [ ] On‑Prem: still use `(client_id=1, user_id)` for the same shapes

## Technical Details
* Files:
  * `src/states.rs`
  * `src/handlers/user.rs` (extend with FSM logic)
  * `src/main.rs` (add FSM middleware)
* Documentation: `docs/FSM_GUIDE.md`, `docs/LOGGING.md`.
* Libraries: `tokio` (for timers), `teloxide` (for user ID type).
* Data types:
  * `UserId` — from `teloxide::types::UserId`.
  * `DateTime<Utc>` — from `chrono` for timeout tracking.

## Acceptance Criteria
* When `/appointment` is triggered, the bot transitions to `SelectingDate`.
* The user can proceed through all booking states sequentially.
* If no action is taken for 10 minutes, the state resets to `Idle`.
* Each state change is logged with `tracing::info!`.
* The bot does not accept inputs intended for other states (e.g., phone number in `SelectingDate` state).
* In SaaS mode, two tenants can have the same Telegram user ID without sharing session/state.

## Prompt for AI Agent
Based on `docs/FSM_GUIDE.md` and `docs/LOGGING.md`, implement task 0004.


1. In `src/states.rs`, define the enum `AppointmentState`. Include all states listed in `docs/FSM_GUIDE.md`. Add `Debug` and `Clone` derive attributes. Add a comment above each variant describing its purpose.
2. Create a struct `UserSession` with fields:
   * `state: AppointmentState`
   * `appointment_data: AppointmentDraft` (partial data collected so far)
   * `last_activity: DateTime<Utc>`
3. In `src/handlers/user.rs`, extend the `/appointment` handler:
   * When triggered, set user’s state to `SelectingDate`.
   * Send a message with a calendar keyboard (use `calendar_keyboard()` from `src/keyboards.rs`).
   * Log the state change.
4. Implement a timeout mechanism:
   * Use `tokio::spawn` to start a background task per user.
   * After 10 minutes of inactivity, reset the state to `Idle` and send a message: «Booking session expired. Use /appointment to start again».
5. Ensure logging for all state transitions using `tracing::info!("State changed: {} -> {}", old_state, new_state)`.
