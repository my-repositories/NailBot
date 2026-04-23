# NailBot: Telegram Bot for Nail Appointment Booking

An asynchronous Telegram bot written in Rust for booking appointments with a nail technician.

## Distribution Model (Hybrid)

This repository is designed to support two modes from one core codebase:

- **SaaS (Multi-tenant)**: one hosted instance serves multiple client bots/tenants with optional subscription/billing logic.
- **On‑Premise (Single-tenant)**: a standalone, customer-hosted instance (source code sale) with license key verification.

## Technologies

* **Language:** Rust (1.65+)
* **Framework:** teloxide
* **Async Runtime:** tokio
* **Database:** SQLite
* **Date/Time:** chrono
* **Configuration:** dotenvy
* **Logging:** tracing

## Features

**For Users:**
* Appointment booking via calendar
* Time slot selection
* Name and phone input
* View/cancel appointments
* Subscription verification
* Price list & portfolio

**For Administrators:**
* Schedule management
* Close days for bookings
* View/cancel client appointments
* Receive notifications

**Automation:**
* 24‑hour reminders
* Task recovery after restart

## Installation

1. Clone: `git clone <url>`
2. `cp .env.onprem.example .env`
3. Fill `.env` (at minimum: MODE, BOT_TOKEN, ADMIN_IDS, DATABASE_URL)
4. `cargo build`
5. `cargo run`

## Structure

See:

- `docs/ARCHITECTURE.md` (modular hybrid architecture)
- `docs/SETUP.md` (env strategy for SaaS vs on‑prem)
- `docs/DB_SCHEMA.md` (tenant-safe schema)
- `docs/DEPLOY.md` (Docker/Compose deployment + licensing concept)

## Contributing

Issues and PRs welcome. Follow code style.
