# Task 0007: Implement User Data Collection


## Goal
Enable the bot to collect user data (name, phone number) during the booking process and store it in the FSM state.

## Context
Seventh task in the MVP phase (`ROADMAP.md`). Follows time slot selection implementation. Allows the bot to gather necessary contact information before confirmation.

## Requirements
* [ ] Implement handler for `InputtingName` state in `src/handlers/user.rs`
* [ ] Prompt user to enter their full name
* [ ] Validate name input (non‑empty, reasonable length)
* [ ] Store name in `UserSession.appointment_data`
* [ ] Implement handler for `InputtingPhone` state
* [ ] Prompt user to enter phone number
* [ ] Validate phone format (basic check: digits, +, spaces, hyphens)
* [ ] Store phone in `UserSession.appointment_data`
* [ ] Add «Back» functionality to return to previous step

## Technical Details
* Files:
  * `src/handlers/user.rs` (extend with data collection handlers)
  * `src/database/models.rs` (update `AppointmentDraft` struct)
* Documentation: `docs/FSM_GUIDE.md`, `docs/VALIDATION_RULES.md`
* Libraries: `teloxide`, `regex` (for phone validation)
* Key considerations:
  * Use `Message::text()` to capture user input
  * Handle edge cases: empty messages, wrong format
  * Store partial data even if user goes back and forth between steps
  * Log all user inputs (with PII redaction for phone numbers)

## Acceptance Criteria
* In `InputtingName` state:
  * Bot sends message: «Please enter your full name:»
  * Valid name (1–50 chars) advances state to `InputtingPhone`
  * Invalid input (empty/too long) triggers error: «Name is required (max 50 chars). Please try again.»
* In `InputtingPhone` state:
  * Bot sends message: «Please enter your phone number:»
  * Valid phone (matches basic pattern) advances state to `Confirming`
  * Invalid phone triggers error: «Invalid phone format. Please enter a valid number.»
* «Back» button:
  * Returns to previous state (`InputtingName` ← `InputtingPhone`)
  * Preserves previously entered data
* All inputs are logged (phone redacted as `+***-***-***-1234`)

## Prompt for AI Agent
Based on `docs/FSM_GUIDE.md` and `docs/VALIDATION_RULES.md`, implement task 0007.


1. In `src/database/models.rs`, update the `AppointmentDraft` struct to include:
   * `name: Option<String>`
   * `phone: Option<String>`
2. In `src/handlers/user.rs`, extend the FSM handlers:
   * For `InputtingName`:
     * Send prompt: «Please enter your full name:»
     * On text input:
       * Validate length (1–50 chars)
       * If valid: store in `appointment_data.name`, advance to `InputtingPhone`, log action
       * If invalid: send error message, stay in `InputtingName`
   * For `InputtingPhone`:
     * Send prompt: «Please enter your phone number:»
     * On text input:
       * Validate with regex: `r"^[+]?[0-9\s\-]{10,15}$"`
       * If valid: store in `appointment_data.phone`, advance to `Confirming`, log (redacted)
       * If invalid: send error, stay in `InputtingPhone`
3. Add a «🔙 Back» button to both states:
   * Callback: `back_to_previous`
   * Handler logic:
     * Decrement state (e.g., `InputtingPhone` → `InputtingName`)
     * Preserve existing `appointment_data` fields
     * Send appropriate prompt for new state
4. Ensure all state transitions and inputs are logged via `tracing::info!`, with phone numbers redacted.
