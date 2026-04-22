# Task 0006: Implement Time Slots Selection

## Goal
Create a time slot selection interface that allows users to choose a free time slot for their appointment after selecting a date. Integrate with the modular keyboard system (`src/keyboards/`).

## Context
Sixth task in the MVP phase (`ROADMAP.md`). Follows calendar keyboard implementation. Enables users to complete the date‑and‑time selection step in the booking flow.

## Requirements
* [ ] Create a time slots keyboard module in `src/keyboards/time_slots.rs`
* [ ] Implement function `time_slots_keyboard(date: &str)` that generates buttons for free slots
* [ ] Query the database for available time slots on the selected date
* [ ] Display only free slots (not already booked)
* [ ] Include «Back» button to return to calendar
* [ ] Add «Reschedule Date» button to change selected date
* [ ] Generate callback data in format: `time_select:YYYY-MM-DD HH:MM`
* [ ] Format time slots as «09:00», «09:30», etc.

## Technical Details
* Files:
  * `src/keyboards/time_slots.rs` — time slots keyboard logic
  * `src/keyboards/mod.rs` — update to include time slots module
  * `src/handlers/user.rs` — extend with time slot handlers
  * `src/database/queries.rs` — add time slot queries
* Documentation: `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, `docs/UI_GUIDE.md`, `docs/BUSINESS_RULES.md`
* Libraries: `teloxide`, `rusqlite`, `chrono`, `regex`
* Key considerations:
  * Use business hours and interval from `docs/BUSINESS_RULES.md` (e.g., 09:00–20:00, 30‑min intervals)
  * Query `appointments` table to check for conflicts
  * Handle timezone: all times are in bot’s local timezone (UTC+3)
  * Ensure keyboard updates dynamically based on availability

## Acceptance Criteria
* Time slot keyboard displays after date selection
* Only free slots are shown (no overlaps with existing appointments)
* Slots are formatted as «09:00», «09:30», etc., in chronological order
* Clicking a slot:
  * sends callback `time_select:YYYY-MM-DD HH:MM`
  * advances FSM to `InputtingName` state
  * logs the selection
* «Back» returns to calendar view (`SelectingDate` state) with current month
* «Reschedule Date» returns to date selection, showing current month
* If no slots are available, show message: «No free slots on {date}. Please select another date.» and display calendar
* Keyboard updates correctly when returning from other states


## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, `docs/UI_GUIDE.md`, and `docs/BUSINESS_RULES.md`, implement task 0006.

1. Update `src/keyboards/mod.rs` to include the time slots module:
   ```rust
   // File: src/keyboards/mod.rs
   pub mod main_menu;
   pub mod calendar;
   pub mod time_slots;

   // Re‑export for convenience
   pub use main_menu::main_menu;
   pub use calendar::calendar_keyboard;
   pub use time_slots::time_slots_keyboard;
   ```
2. Create `src/keyboards/time_slots.rs`:
   ```rust
   // File: src/keyboards/time_slots.rs
   use teloxide::types::KeyboardMarkup;

   pub fn time_slots_keyboard(date: &str) -> KeyboardMarkup {
       // Implementation of time slots grid generation
       // - Call get_available_time_slots(date) to get free slots
       // - Create buttons for each slot with callback time_select:{date} {slot}
       // - Add «🔙 Back» (callback: back_to_calendar)
       // - Add «🗓️ Reschedule Date» (callback: reschedule_date)
       // Return KeyboardMarkup object
   }
   ```
3. In `src/database/queries.rs`, extend with function `get_available_time_slots(date: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>`:
   * Query `appointments` table for booked slots on `date`
   * Generate full list of possible slots using business rules
   * Return `Vec` of free slots in «HH:MM» format
4. In `src/handlers/user.rs`, add handlers for time slot interactions:
   * Handler for `time_select:*` callbacks:
     * Parse date and time from callback data
     * Update user’s FSM state to `InputtingName`
     * Store selected time in `appointment_data`
     * Log the time selection
     * Send message: «Time selected: {time}. Now please provide your name.»
   * Handlers for `back_to_calendar` and `reschedule_date`:
     * `back_to_calendar`: return to `SelectingDate`, show calendar for selected date
     * `reschedule_date`: reset to date selection, show current month calendar
5. Ensure all state transitions and user actions are logged via `tracing::info!`.
