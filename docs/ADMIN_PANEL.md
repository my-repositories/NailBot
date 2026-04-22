# Admin Panel Guide

## Access

* Command: `/admin`
* Check `ADMIN_ID` from config.
* Show admin menu if authorized.

## Admin Commands

* `/add_day <YYYY-MM-DD>` — Add working day.
* `/remove_day <YYYY-MM-DD>` — Remove day.
* `/close_day <YYYY-MM-DD>` — Close for bookings.
* `/view_schedule <YYYY-MM-DD>` — Show schedule.
* `/cancel_appointment <ID>` — Cancel client's appointment.
* `/stats` — Show booking statistics.

## Handler Rules

* All handlers are `async fn`.
* Return `Result<(), Box<dyn std::error::Error>>`.
* Log admin actions with level `INFO`.
* Send confirmation messages after operations.

## Prompts for AI

> Implement `/admin` handler in `src/handlers/admin.rs`. Check `ADMIN_ID`. Show inline menu with buttons: «Add Day», «View Schedule», «Stats». Use `tracing` to log access attempts.

> Create `handle_add_day` function. Parse date from command. Add to `time_slots` table. Send confirmation. Handle invalid dates.

## UI Rules

* Admin messages: bold headers, clear statuses.
* Use emoji for actions: 🗓️ Add, 📊 Stats, ❌ Cancel.
* Limit message length (4096 chars).
