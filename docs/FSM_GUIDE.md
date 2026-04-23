# FSM Guide

Hybrid requirement:

- **SaaS (multi-tenant)**: FSM/session storage must be keyed by **tenant + user**, e.g. `(client_id, telegram_id)`.
- **On‑Prem (single-tenant)**: a single implicit tenant (e.g., `client_id = 1`) still uses the same shapes, keeping codepaths unified.

## States

* `Idle` — Default state.
* `SelectingDate` — User selects date.
* `SelectingTime` — User selects time slot.
* `InputtingName` — User inputs name.
* `InputtingPhone` — User inputs phone.
* `Confirming` — User confirms details.
* `AdminPanel` — Admin menu state.

## Transitions

1. `/start` → `Idle`
2. `Select Date` → `SelectingDate`
3. `Date Selected` → `SelectingTime`
4. `Time Selected` → `InputtingName`
5. `Name Entered` → `InputtingPhone`
6. `Phone Entered` → `Confirming`
7. `Confirmed` → Save to DB → `Idle`
8. `/admin` → `AdminPanel`

## Rules

* Store state per user.
  - SaaS: key = `(client_id, telegram_id)`
  - On‑Prem: key = `(1, telegram_id)`
* Timeout: 10 minutes to return to `Idle`.
* On timeout: send message and reset state.
* Log state changes with `tracing`.

## AI Prompts

> Define FSM states in `src/states.rs`. Use enum `AppointmentState` with variants: Idle, SelectingDate, SelectingTime, InputtingName, InputtingPhone, Confirming, AdminPanel. Derive Debug, Clone. Add comments for each state.

> Implement state timeout logic. Use `tokio::time::sleep`. Send «Session expired» message after 10 minutes. Reset to `Idle`.