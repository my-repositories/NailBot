# Architecture Overview (Hybrid Distribution Model)

This codebase is designed to support **two distribution modes** from a shared core:

- **SaaS (Multi-tenant)**: one hosted instance serves many client bots/tenants, keyed by `client_id`, with optional subscription/billing logic.
- **On‑Premise (Single-tenant)**: a standalone instance serves exactly one tenant, deployed by the customer (source sale) with **license key verification**.

The key architectural goal is **modularity**: keep booking/admin logic stable while supporting multiple client channels (Telegram now, Web/Mobile next) through one backend contract.

## Modular Architecture (API-first)

### Client Adapters (transport layer)

**Responsibility**: receive channel-specific input and call backend API/application contracts.

- **Telegram bot adapter**: update parsing, FSM/dialog state, message rendering.
- **Future adapters**: web frontend and mobile app clients.

**Design constraints**:

- No direct DB access from adapters.
- No business rule duplication in adapters.
- Map backend errors to channel-specific UX only.

### API Layer (HTTP contract)

**Responsibility**: expose versioned endpoints consumed by bot/web/mobile clients.

- Request/response DTOs
- Auth/context extraction (including tenant context)
- Error contract for all clients

### Business Logic Core (application + domain)

**Responsibility**: all booking/admin rules, independent of transport, storage, and distribution mode.

- **Use-case flows**: booking, admin operations, reminders, subscription policy checks.
- **Domain invariants**: slot availability, cancellation rules, working hours, pricing rules.
- **Use-cases**: “Create appointment”, “List appointments”, “Close day”, “Send reminder”.

**Design constraints**:

- No direct access to env vars.
- No direct SQL queries.
- No Telegram/Web/Mobile I/O types in domain/application layers.

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

1. **Telegram flow**: user update → bot adapter/FSM → API endpoint → use-case → repository → API response → bot message.
2. **Web/mobile flow**: client request → API endpoint → use-case → repository → API response.
3. **Booking/Admin behavior** stays consistent across channels because rules live in application/domain layers.

## Implementation Task Anchor

The separation work is tracked in:

- `docs/tasks/0000-separate-api-service-and-bot-handler.md`

## Dependencies (Typical)

- **Telegram API**: `teloxide`
- **Async runtime**: `tokio`
- **DB**: `rusqlite` (SQLite) and/or `sqlx`/`tokio-postgres` (Postgres) depending on adapter choice
- **Date/time**: `chrono` / `time`
- **Config**: `dotenvy` + typed validation (`config`, `serde`, or custom parsing)
