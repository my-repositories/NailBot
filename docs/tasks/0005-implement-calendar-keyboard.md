# Task 0005: Implement Calendar Keyboard

## Goal
Create an interactive calendar keyboard that allows users to select a date for their appointment.

## Context
Fifth task in the MVP phase (`ROADMAP.md`). Follows FSM implementation. Enables users to visually select a date during the booking flow (`SelectingDate` state). Uses the modular keyboard system (`src/keyboards/`).

## Requirements
* [ ] Create a calendar keyboard module in `src/keyboards/calendar.rs`
* [ ] Implement function `calendar_keyboard(year: i32, month: u32)` that generates a monthly grid
* [ ] Display days of the month in a weekly grid (Sunday/Monday start per `docs/UI_GUIDE.md`)
* [ ] Highlight the current date visually (with emoji)
* [ ] Add navigation buttons: «⬅️ Previous Month», «Next Month ➡️»
* [ ] Include a «Back to Menu» button
* [ ] Generate callback data for each day in format: `date_select:YYYY-MM-DD`
* [ ] Ensure the keyboard adapts to month length and starting weekday

## Technical Details
* Files:
  * `src/keyboards/calendar.rs` — calendar keyboard logic
  * `src/keyboards/mod.rs` — update to include calendar module
  * `src/handlers/user.rs` — extend with calendar handlers
* Documentation: `docs/ARCHITECTURE.md`, `docs/UI_GUIDE.md`, `docs/FSM_GUIDE.md`
* Libraries: `teloxide`, `chrono`
* Key considerations:
  * Use `chrono::NaiveDate` for date calculations
  * Format dates as `YYYY-MM-DD` in callback data
  * Handle month/year overflow (e.g., January 2024 → December 2023)
  * Limit calendar to current and next 3 months
  * Ensure proper week alignment (start of week per `docs/UI_GUIDE.md`)

## Acceptance Criteria
* Calendar displays correct number of days for selected month (28–31)
* Days are arranged in a weekly grid with correct starting day
* Current date is visually distinct (e.g., marked with 🟢)
* Navigation buttons update the calendar display when clicked
* Clicking a day:
  * sends callback `date_select:YYYY-MM-DD`
  * advances FSM to `SelectingTime` state
  * logs the selection
* «Back to Menu» returns user to main menu and resets FSM state to `Idle`
* Keyboard updates correctly when navigating between months
* SaaS tenant isolation: callbacks cannot be used to drive state for a different tenant (session keyed by `client_id`)

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md`, `docs/UI_GUIDE.md` and `docs/FSM_GUIDE.md`, implement task 0005.

1. Update `src/keyboards/mod.rs` to include the calendar module:
   ```rust
   // File: src/keyboards/mod.rs
   pub mod main_menu;
   pub mod calendar;

   // Re‑export for convenience
   pub use main_menu::main_menu;
   pub use calendar::calendar_keyboard;
   ```
2. Create `src/keyboards/calendar.rs`:
   ```rust
   // File: src/keyboards/calendar.rs
   use teloxide::types::KeyboardMarkup;
   use chrono::{NaiveDate, Datelike};

   pub fn calendar_keyboard(year: i32, month: u32) -> KeyboardMarkup {
       // Implementation of calendar grid generation
       // - Calculate first day of month and number of days
       // - Create rows of buttons (7 days per row)
       // - Add navigation row with month controls
       // - Add final row with «Back to Menu»
       // Return KeyboardMarkup object
   }
   ```
3. In `src/handlers/user.rs`, add handlers for calendar interactions:
   * Handler for `date_select:*` callbacks:
     * Parse the selected date from callback data
     * Update user’s FSM state to `SelectingTime`
     * Log the date selection
     * Send confirmation message: «Date selected: {date}. Now choose a time slot.»
   * Handlers for `nav_prev:*` and `nav_next:*`:
     * Calculate previous/next month (handle year overflow)
     * Regenerate calendar with `calendar_keyboard()`
     * Edit the message with the new keyboard
   * Handler for `back_to_menu`:
     * Reset FSM state to `Idle`
     * Show main menu via `keyboards::mainmenu::mainmenu()` keyboard
4. Ensure all state transitions and user actions are logged via `tracing::info!`.
