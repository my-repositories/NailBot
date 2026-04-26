# NailBot: Booking Platform Core (API + Telegram Bot)

An asynchronous Rust project for appointment booking where Telegram is one client channel and Web API is the system core for future channel expansion.

## Distribution Model (Hybrid)

This repository supports two modes from one core codebase:

- **SaaS (Multi-tenant)**: one hosted platform serves multiple tenants by `client_id`.
- **On‑Premise (Single-tenant)**: one customer-hosted tenant with license key verification.

## Architecture Direction

The target architecture is separated into:

- **Web API service**: owns business logic, validation, and data access.
- **Bot handler**: Telegram transport adapter that calls the Web API.

Why this split:

- Reuse the same booking backend for web and mobile apps.
- Keep business rules in one place.
- Allow independent scaling and deployment of API and bot workers.

## Technologies

* **Language:** Rust (1.65+)
* **Framework:** teloxide
* **Async Runtime:** tokio
* **Database:** PostgreSQL
* **Date/Time:** chrono
* **Configuration:** dotenvy
* **Logging:** tracing

## Features

**For end users (via Telegram today):**
* Appointment booking via calendar
* Time slot selection
* Name and phone input
* View/cancel appointments
* Subscription verification
* Price list & portfolio

**For administrators:**
* Schedule management
* Close days for bookings
* View/cancel client appointments
* Receive notifications

**Automation:**
* 24‑hour reminders
* Task recovery after restart

## Run

This project now runs as **two separate applications**:

- `nailbot-api` - HTTP API service
- `nailbot-bot` - Telegram worker service

### Local run (PowerShell)

1. `Copy-Item .env.onprem.example .env` (or use `.env.saas.example`)
2. `./scripts/run-api.ps1`
3. In another terminal: `./scripts/run-bot.ps1`

### Local run (Bash/Shell)

1. `cp .env.onprem.example .env` (or use `.env.saas.example`)
2. `./scripts/run-api.sh`
3. In another terminal: `./scripts/run-bot.sh`

### Docker Compose

1. Ensure `.env` exists
2. `docker compose up -d --build`
3. Check services: `docker compose ps`

## i18n Baseline (RU/EN)

- Runtime localization is based on `fluent` + `fluent-bundle`.
- Locale identifiers and fallback use `unic-langid`.
- Catalogs are externalized in `locales/en.ftl` and `locales/ru.ftl`.
- SaaS resolves locale per user (tenant-scoped).
- On‑Prem uses env default locale with optional per-user override.
- Supported locale values are normalized to `en` / `ru` (for example, `en-US` -> `en`, `ru-RU` -> `ru`).
- Bot resolves locale through API (`/api/v1/users/{telegram_id}/locale`) with fallback to `DEFAULT_LOCALE`.

## Structure

See:

- `docs/ARCHITECTURE.md` (API-first modular hybrid architecture)
- `docs/SETUP.md` (env strategy for API and bot by mode)
- `docs/DB_SCHEMA.md` (tenant-safe schema)
- `docs/DEPLOY.md` (API/bot deployment patterns + licensing concept)
- `docs/tasks/0000-separate-api-service-and-bot-handler.md` (separation task definition)

## Contributing

Issues and PRs welcome. Follow code style.
