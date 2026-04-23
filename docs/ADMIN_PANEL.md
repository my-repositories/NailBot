# Admin Panel Guide

Applies to both modes, but with different scope:

- **On‑Prem (single-tenant)**: admins manage one tenant. `ADMIN_IDS` is a single list.
- **SaaS (multi-tenant)**: there are typically **tenant admins** (per `client_id`) and optionally **platform admins** (SaaS-only) who can manage the whole fleet.

## Access

* Command: `/admin`
* Check `ADMIN_IDS` from config (on‑prem) or per-tenant admin list (SaaS).
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
* **Tenant isolation (SaaS)**: every admin query/update must be scoped to the current `client_id`.

## Prompts for AI

> Implement `/admin` handler in `src/handlers/admin.rs`. Check admin authorization (on‑prem: `ADMIN_IDS`; SaaS: per-tenant admins). Show inline menu with buttons: «Add Day», «View Schedule», «Stats». Log access attempts including `client_id` (SaaS).

> Create `handle_add_day` function. Parse date from command. Add to `time_slots` table. Send confirmation. Handle invalid dates.

## UI Rules

* Admin messages: bold headers, clear statuses.
* Use emoji for actions: 🗓️ Add, 📊 Stats, ❌ Cancel.
* Limit message length (4096 chars).

## SaaS-only admin features (mark as SaaS-only in tasks)

- Cross-tenant admin dashboard
- Tenant lifecycle management (create/suspend/delete tenant)
- Global billing/entitlements
