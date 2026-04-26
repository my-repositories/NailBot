# Task 0011: Implement Admin Panel & Notifications

## Goal
Create an administrative interface for staff to monitor appointments, receive notifications, and view business metrics.


## Context
Eleventh task in the MVP phase (`ROADMAP.md`). Follows user appointment management. Enables salon staff to oversee bookings and respond to new appointments.

## API/Bot Split Alignment
* [ ] Admin data and mutations (stats, mark-done, today list) are served by API endpoints.
* [ ] Bot admin commands are transport adapters over API operations.
* [ ] Tenant-scoped admin authorization is enforced in API, not only in bot command guards.


## Requirements
* [ ] Implement admin‑only commands: `/admin`, `/stats`
* [ ] Restrict access via admin user IDs (from config)
* [ ] Send notifications to admin on new appointment
* [ ] Display daily/weekly booking statistics
* [ ] Show list of today’s appointments with client details
* [ ] Add «Mark as Done» button for completed appointments
* [ ] Include «Back to Menu» navigation
* [ ] Log admin actions

## Technical Details
* Files:
  * `src/handlers/admin.rs` (new — admin handlers)
  * `src/database/queries.rs` (extend with admin queries)
  * `src/keyboards/admin.rs` (new — admin keyboards)
  * `src/config.rs` (add `ADMIN_IDS` array)
* Documentation: `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, `docs/ADMIN_GUIDE.md`
* Libraries: `teloxide`, `rusqlite`, `chrono`
* Key considerations:
  * Use `ADMIN_IDS` from config to check permissions
  * Ensure client PII (phone) is visible only to admins
  * Format dates/times in local timezone (UTC+3)
  * Cache stats calculations for performance
  * Rate‑limit notification messages
  * Hybrid scoping:
    * On‑Prem: `ADMIN_IDS` is global (single tenant)
    * SaaS: admin authorization is per-tenant; do not leak cross-tenant data

## Acceptance Criteria
* `/admin` command:
  * Accessible only to users in `ADMIN_IDS`
  * If unauthorized: sends «❌ Access denied» and logs attempt
  * If authorized: shows admin dashboard with buttons:
    * «📊 Today’s Stats»
    * «🗓️ Today’s Appointments»
    * «📈 Weekly Overview»
    * «⚙️ Settings» (future use)
* New appointment notification:
  * Sent to all admin IDs immediately after booking
  * Message:
    ```
    🔔 New Appointment!

    👤 Client: {name}
    📱 Phone: {phone}
    🗓️ Date: {date}
    ⏰ Time: {time}
    ```
* `/stats` command:
  * Shows:
    * Total bookings (today/week)
    * Cancellation rate
    * Busiest hours
    * Load by service type
* «Today’s Appointments»:
  * Lists all confirmed appointments for current day
  * For each: client name, time, service
  * Inline button «✅ Mark as Done» (callback: `mark_done:{appointment_id}`)
* «Mark as Done»:
  * Updates DB: `status = 'completed'`
  * Removes from active lists
  * Logs action with admin ID
* All admin actions are logged via `tracing::info!`

## SaaS-only extensions (explicitly SaaS-only)

* [ ] (SaaS-only) Platform admin dashboard (cross-tenant): uptime, tenant list, error rates
* [ ] (SaaS-only) Tenant lifecycle: create/suspend/reactivate tenants
* [ ] (SaaS-only) Global billing/entitlements integration (if enabled)

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, and `docs/ADMIN_GUIDE.md`, implement task 0011.

1. Update `src/config.rs`:
   ```rust
   // File: src/config.rs
   pub const ADMIN_IDS: [i64; 2] = [123456789, 987654321]; // Example IDs
   ```
2. Create `src/keyboards/admin.rs`:
   ```rust
   // File: src/keyboards/admin.rs
   use teloxide::types::{InlineKeyboardMarkup, InlineKeyboardButton};

   pub fn admin_dashboard_keyboard() -> InlineKeyboardMarkup {
       InlineKeyboardMarkup::new(vec![
           vec![InlineKeyboardButton::callback("📊 Today's Stats", "stats_today".to_string())],
           vec![InlineKeyboardButton::callback("🗓️ Today's Appointments", "today_appointments".to_string())],
           vec![InlineKeyboardButton::callback("📈 Weekly Overview", "weekly_stats".to_string())],
           vec![InlineKeyboardButton::callback("⚙️ Settings", "admin_settings".to_string())]
       ])
   }

   pub fn appointments_list_keyboard(appointments: &[Appointment]) -> InlineKeyboardMarkup {
       let mut rows = vec![];
       for appt in appointments {
           rows.push(vec![InlineKeyboardButton::callback(
               format!("✅ {} at {}", appt.name, appt.time),
               format!("mark_done:{}", appt.id)
           )]);
       }
       rows.push(vec![InlineKeyboardButton::callback("🔙 Back", "back_to_admin".to_string())]);
       InlineKeyboardMarkup::new(rows)
   }
   ```
3. In `src/database/queries.rs`, add:
   * `get_today_appointments() -> Result<Vec<Appointment>, _>`
   * `get_weekly_stats() -> Result<AdminStats, _>`
   * `mark_appointment_done(id: i64) -> Result<(), _>`
4. Create `src/handlers/admin.rs`:
   * Handler for `/admin`:
     * Check `user_id` against `ADMIN_IDS`
     * Send dashboard message with `admin_dashboard_keyboard()`
   * Handler for `stats_today`:
     * Call `get_today_stats()`
     * Format and send message
   * Handler for `today_appointments`:
     * Fetch today’s list
     * Attach `appointments_list_keyboard()`
   * Handler for `mark_done:*`:
     * Parse ID, call `mark_appointment_done()`
     * Refresh list
     * Log: `tracing::info!("Appointment {} marked done by admin {}", id, admin_id)`
5. In `src/handlers/user.rs`, after saving a new appointment:
   * Loop through `ADMIN_IDS`
   * Send notification message to each
6. Ensure all admin actions and notifications are logged with sufficient context.
