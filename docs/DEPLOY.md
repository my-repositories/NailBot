# Deployment Guide: Appointment Booking Bot


Follow these steps to deploy the bot in production.

## Prerequisites
* **Rust** (1.70+) — for building the application.
* **Database** — choose one:
  * **PostgreSQL 12+** (recommended for production).
  * **SQLite 3.30+** (simpler setup, suitable for small scale).
* **Telegram Bot Token** — get from @BotFather.
* **Admin Telegram IDs** — for access control (at least one admin).
* **System User** — create a dedicated user for the bot (e.g., `botuser`).

## Step 1: Clone the Repository
```bash
git clone https://github.com/salon/appointment-bot.git
cd appointment-bot
```

## Step 2: Configure the Bot
1. Copy the example config:
```bash
cp config.example.toml config.toml
```
2. Edit `config.toml`:
```toml
telegram_bot_token = "1234567890:ABCdefGHIjklMNOpqrsTUVwxyz"
database_url = "sqlite:///app/data/bot.db"
# Or for PostgreSQL:
# database_url = "postgres://username:password@localhost/appointments"
admin_ids = [123456789, 987654321]
timezone = "Europe/Moscow"
log_level = "info"
```

## Step 3: Set Environment Variables (Optional)
You can override config values with environment variables:
```bash
export TELEGRAM_BOT_TOKEN="1234567890:ABCdefGHIjklMNOpqrsTUVwxyz"
export DATABASE_URL="sqlite:///app/data/bot.db"
```

## Step 4: Build the Application
```bash
cargo build --release
```

## Step 5: Run the Bot

### Option A: Manual Run
```bash
./target/release/appointment_bot
```

### Option B: Using systemd (Recommended for Production)
1. Create a systemd service file:
```ini
# File: /etc/systemd/system/appointment-bot.service
[Unit]
Description=Appointment Booking Bot
After=network.target

[Service]
Type=simple
User=botuser
WorkingDirectory=/opt/appointment-bot
ExecStart=/opt/appointment-bot/target/release/appointment_bot
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```
2. Enable and start the service:
```bash
sudo systemctl daemon-reload
sudo systemctl enable appointment-bot.service
sudo systemctl start appointment-bot.service
```
3. Check the status:
```bash
sudo systemctl status appointment-bot.service
```

## Step 6: Verify Operation
1. Send `/start` to your bot in Telegram.
2. Check logs for startup messages:
```log
INFO bot started, listening for updates
INFO admin panel ready for IDs: [123456789]
```
3. Test booking flow: `/start` → book an appointment → check DB.
4. Verify admin notifications: book an appointment and check if admins receive a notification.

## Maintenance
* Monitor logs: `journalctl -u appointment-bot -f` (if using systemd).
* Update the bot:
  1. Pull new code: `git pull`.
  2. Rebuild: `cargo build --release`.
  3. Restart the service: `sudo systemctl restart appointment-bot.service`.
* Backup DB regularly:
  * For SQLite: copy `bot.db` file.
  * For PostgreSQL: use `pg_dump`.

## Troubleshooting

**Bot won’t start:**
* Check `config.toml` syntax.
* Verify DB URL and permissions.
* Ensure the Telegram token is correct.
* Confirm the bot has read/write access to the DB file/directory.

**Appointments not saving:**
* Confirm DB user has write permissions.
* Check disk space.
* Verify the bot process has the correct working directory.

**No admin notifications:**
* Verify `ADMIN_IDS` are correct Telegram IDs.
* Test sending a message to each admin ID manually.
* Check bot logs for error messages.

**High memory usage:**
* Monitor with `top` or `htop`.
* Consider using PostgreSQL instead of SQLite for large datasets.
* Review the bot’s logging level (`log_level` in config).

**Slow response times:**
* Optimize DB indexes.
* Increase system resources (CPU/RAM).
* Check for long‑running queries.

## Security Best Practices
* Never expose the Telegram bot token in public repositories.
* Use a dedicated system user (`botuser`) with minimal permissions.
* Keep Rust and system packages up to date.
* Restrict access to the config file (`chmod 600 config.toml`).
* Regularly rotate the Telegram bot token if compromised.

## Contact
For deployment issues, contact the ops team:
* Email: ops@salon.com
* Telegram: @ops_support_bot
```
