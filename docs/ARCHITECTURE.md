# Architecture Overview (Hybrid Distribution Model)

This codebase is designed to support **two distribution modes** from a shared core:

- **SaaS (Multi-tenant)**: one hosted instance serves many client bots/tenants, keyed by `client_id`, with optional subscription/billing logic.
- **On‑Premise (Single-tenant)**: a standalone instance serves exactly one tenant, deployed by the customer (source sale) with **license key verification**.

The key architectural goal is **modularity**: keep your booking/admin logic stable while swapping configuration, database backends, and distribution-specific features.

## Modular Architecture

### Business Logic Core (domain + use-cases)

**Responsibility**: all “what the bot does” rules, independent of storage and deployment mode.

- **Conversation + flows**: booking flow, admin flow, reminders logic, subscription check policy.
- **Domain invariants**: slot availability, cancellation rules, working hours, pricing rules.
- **Use-cases**: “Create appointment”, “List appointments”, “Close day”, “Send reminder”.

**Design constraints**:

- No direct access to env vars.
- No direct SQL queries.
- No Telegram I/O types in the domain layer; use thin adapters/interfaces.

### Database Adapters (ports + implementations)

**Responsibility**: persistence and queries behind a narrow interface used by the core.

- **Port (interface)**: repository traits like `UsersRepo`, `AppointmentsRepo`, `SlotsRepo`.
- **Adapters**: `sqlite`, `postgres` implementations; migrations; connection pooling; transaction boundaries.
- **Multi-tenancy**: enforce `client_id` scoping at the adapter boundary (or via row-level policy in Postgres).

### Configuration Layer (runtime wiring)

**Responsibility**: load configuration and select the correct runtime “mode”.

- Reads env vars / config files (if used), validates, and constructs a `Settings` struct.
- Picks **distribution mode**: `MODE=saas` vs `MODE=onprem`.
- In SaaS: loads multiple tenants and their bot tokens.
- In On‑Prem: loads one bot token and enables license verification.

## Distribution Mode Responsibilities

### SaaS (Multi-tenant)

- **Tenant registry**: mapping of `client_id → bot_token (+ settings)`.
- **Request routing**: every incoming update is associated to a tenant (e.g., by which bot token / webhook path / polling instance).
- **Billing/subscription hooks** (optional): entitlement checks before allowing premium features.

### On‑Premise (Single-tenant)

- **Tenant defaulting**: application runs with an implicit single tenant:
  - `client_id = 1` (convention) or `client_id = NULL` (schema-compatible default).
- **License verification**: validate a license key at startup and periodically (see `docs/DEPLOY.md` licensing section).

## Data Flow (Core Flows)

These flows remain the same in both modes; only configuration and persistence wiring differ.

1. **User command** → update received → handler routes → core use-case executes → state changes persisted → response sent.
2. **Booking**: `/start` → menu → date → time → contact data → persist appointment → schedule reminder.
3. **Admin**: `/admin` → panel → manage schedule (close day, open slots) → update DB.

## Dependencies (Typical)

- **Telegram API**: `teloxide`
- **Async runtime**: `tokio`
- **DB**: `rusqlite` (SQLite) and/or `sqlx`/`tokio-postgres` (Postgres) depending on adapter choice
- **Date/time**: `chrono` / `time`
- **Config**: `dotenvy` + typed validation (`config`, `serde`, or custom parsing)
