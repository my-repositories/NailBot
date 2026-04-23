# Validation Rules (Hybrid: SaaS + On‑Prem)

These rules apply in both modes. In **SaaS**, validation must also enforce **tenant isolation** (every read/write is scoped to the current `client_id`).

## Text inputs

### Name

- Required during booking.
- Trim whitespace.
- Length: 1–50 characters.
- Reject control characters.

### Phone

- Required during booking (for confirmations/reminders).
- Accept: digits, spaces, `+`, `-`, parentheses.
- Normalize before storage (recommended): keep `+` and digits only.
- Basic acceptance regex (pre-normalization): `^[+0-9()\\s\\-]{10,25}$`

## Dates and times

- Date format: `YYYY-MM-DD`
- Time format: `HH:MM`
- Only allow booking within configured horizon (e.g., current day + next 3 months).
- Enforce working hours and slot interval per `docs/BUSINESS_RULES.md`.
- Prevent double-booking:
  - unique slot constraint per tenant (`client_id`, `date`, `time`)
  - transactional check + insert/update

## FSM safety

- Reject inputs that don’t match current state (keep state unchanged; show gentle guidance).
- Timeout: if session expires, reset to `Idle` and require restart of flow.

## Tenant isolation (SaaS)

- Any callback payload that contains an entity ID (appointment_id, user_id, etc.) must be verified as belonging to the current `client_id` before acting.
- Never accept a “raw” appointment ID without a tenant-scoped lookup.

