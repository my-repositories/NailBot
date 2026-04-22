# Setup Guide

## Prerequisites

* Rust 1.65+
* SQLite
* Telegram Bot Token

## Steps

1. **Clone repository:**
   ```bash
   git clone <url>
   cd nail_bot
   ```
2. **Create `.env`:**
   ```bash
   cp .env.example .env
   ```
3. **Fill `.env`:**
   * `BOT_TOKEN` — from @BotFather
   * `ADMIN_ID` — your Telegram ID
   * `CHANNEL_ID` — channel for subscription check
   * `CHANNEL_LINK` — public link
4. **Install dependencies:**
   ```bash
   cargo build
   ```
5. **Run in development:**
   ```bash
   cargo run
   ```

## Troubleshooting

* **«No such file or directory»:** Ensure `.env` exists.
* **«Invalid token»:** Verify `BOT_TOKEN` format.
* **«Database locked»:** Close other connections to `db.sqlite`.
