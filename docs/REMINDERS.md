# Reminders Guide (Hybrid: SaaS + On‑Prem)

Reminders are a cross-cutting feature that must respect **tenant isolation**.

## Reminder types

- **24 hours before**
- **1 hour before**

## Scheduling models

### In-process scheduler (MVP-friendly)

- Runs inside the bot process using `tokio::time`.
- Pros: simplest, good for on‑prem and small SaaS.
- Cons: duplicates if you scale horizontally unless you add a distributed lock / DB coordination.

### DB-coordinated scheduler (SaaS-friendly)

- Store “reminder jobs” in DB and claim them atomically.
- Pros: safe with multiple workers.
- Cons: more moving parts.

## Data requirements

Minimum fields needed to send a reminder (tenant-scoped):

- `client_id`
- `appointment_id`
- `user_telegram_id` (or chat id)
- `date`, `time`, `status`
- “sent flags” to prevent duplicates (e.g., `reminded_24h_at`, `reminded_1h_at`) or a separate reminders table

## Tenant isolation rules (SaaS)

- Any scan for upcoming reminders must filter by `client_id`.
- If running multiple tenant bots in one process, route reminders through the correct bot token for that `client_id`.

## Operational notes

- **SQLite** (common on‑prem): keep a single app instance (single writer).
- **Postgres** (recommended SaaS): supports safe coordination and indexing for reminder scans.

