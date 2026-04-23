# Error Handling Guide (Hybrid: SaaS + On‑Prem)

This project runs in two distribution modes:

- **SaaS (multi-tenant)**: one hosted instance serves many tenants. Errors must include `client_id` context and must not leak tenant data across boundaries.
- **On‑Prem (single-tenant)**: one customer-hosted tenant. Errors should be actionable for operators and support, with safe logs.

## Goals

- Keep the bot responsive during transient failures (Telegram API hiccups, DB contention).
- Fail fast on **non-recoverable** boot blockers (invalid config, invalid/expired on‑prem license).
- Provide user-friendly messages while logging enough context for operators.
- Preserve **tenant isolation**: errors and logs must not expose other tenants' identifiers or data.

## Error taxonomy

- **Config / Boot**: missing env vars, invalid values, invalid mode.
- **License (On‑Prem)**: missing/invalid/expired license key.
- **Telegram API**: rate limits, network timeouts, blocked bot, invalid token.
- **Database**: connection failures, migration failures, lock timeouts (SQLite), serialization errors.
- **Domain / Validation**: invalid date/time selection, invalid phone, invalid state transition.

## Retry policy (transient only)

Use exponential backoff with jitter for transient errors:

- Telegram: retry on network timeouts and `429` (respect `retry_after` if available).
- DB: retry on **lock/contention** (SQLite) and transient connection errors (Postgres).
- Never retry on: invalid input, permission errors, missing rows, invalid license, invalid token.

## User-facing messaging

- **Generic**: “⚠️ Something went wrong. Please try again.”
- **Rate limited**: “⏳ We’re temporarily rate-limited by Telegram. Please try again in a minute.”
- **Maintenance** (SaaS): “🛠️ We’re performing maintenance. Please try again shortly.”
- **License** (On‑Prem): “🔑 License issue detected. Please contact your administrator.”

## Logging requirements (include tenancy context)

Every error log entry should include:

- `mode`: `saas` or `onprem`
- `client_id`:
  - SaaS: required and accurate for the tenant handling the update
  - On‑Prem: constant (e.g., `1`)
- `user_telegram_id` / `chat_id` when relevant
- `operation`: short stable string (e.g., `save_appointment`, `get_available_slots`)
- `error_kind`: one of the taxonomy above

### PII rules

- Never log full phone numbers, license keys, or bot tokens.
- Redact phone numbers to last 2–4 digits.
- Redact license keys (hash or last 6 chars).

## Admin notifications

- **SaaS**:
  - Notify **tenant admins** for tenant-scoped failures (e.g., their DB row is inconsistent).
  - Notify **platform admins** for platform failures (DB down, routing broken, tenant registry unavailable).
- **On‑Prem**:
  - Notify local admins (`ADMIN_IDS`) for critical runtime failures.

