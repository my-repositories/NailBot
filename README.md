# NailBot: Telegram Bot for Nail Appointment Booking

An asynchronous Telegram bot written in Rust for booking appointments with a nail technician.

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
2. `cp .env.example .env`
3. Fill `.env` (BOT_TOKEN, ADMIN_ID, CHANNEL_ID)
4. `cargo build`
5. `cargo run`

## Structure

See `docs/ARCHITECTURE.md`

## Contributing

Issues and PRs welcome. Follow code style.
