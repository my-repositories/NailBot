# Documentation Index (Hybrid: SaaS + On‑Prem)

This repository supports two distribution modes from one core codebase:

- **SaaS (multi-tenant)**: multiple tenants identified by `client_id`
- **On‑Prem (single-tenant)**: one tenant (usually `client_id = 1`) with license verification

## Start here

- `docs/ARCHITECTURE.md` — the hybrid architecture and layering constraints
- `docs/SETUP.md` — configuration strategy and env variables by mode
- `docs/DEPLOY.md` — Docker/Compose deployment + licensing concept
- `docs/DB_SCHEMA.md` — tenant-safe schema and migration notes

## Feature guides

- `docs/UI_GUIDE.md` — UI/UX patterns for messages and keyboards
- `docs/HANDLERS_USER.md` — user command handler patterns
- `docs/ADMIN_PANEL.md` — admin command handler patterns
- `docs/FSM_GUIDE.md` — booking FSM states and transitions
- `docs/SCHEDULER.md` — reminder scheduling approach
- `docs/SUBSCRIPTION.md` — subscription checks (tenant-aware in SaaS)

## Cross-cutting concerns

- `docs/VALIDATION_RULES.md` — input validation and tenant-safety checks
- `docs/REMINDERS.md` — reminder system requirements and scaling notes
- `docs/ERROR_HANDLING.md` — error taxonomy, retries, logging context
- `docs/LOGGING.md` — logging levels and conventions
- `docs/TESTING.md` — testing strategy (include SaaS/on‑prem coverage)

