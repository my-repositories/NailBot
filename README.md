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
* **Database:** SQLite
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

## Run (Current Documentation Target)

Today this repository is documentation-first. Implementation is tracked by task docs.

When implementation starts, expected local flow is:

1. Clone: `git clone <url>`
2. Create env file (mode-specific)
3. Start API + bot (single process in dev or separate services)

## Structure

See:

- `docs/ARCHITECTURE.md` (API-first modular hybrid architecture)
- `docs/SETUP.md` (env strategy for API and bot by mode)
- `docs/DB_SCHEMA.md` (tenant-safe schema)
- `docs/DEPLOY.md` (API/bot deployment patterns + licensing concept)
- `docs/tasks/0000-separate-api-service-and-bot-handler.md` (separation task definition)

## Contributing

Issues and PRs welcome. Follow code style.
