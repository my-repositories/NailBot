# Admin Guide: Appointment Management Panel

This guide is for staff members who manage appointments and monitor the system.

## Accessing the Admin Panel
1. Send `/admin` to the bot.
2. If you’re an authorized admin (configured via `ADMIN_IDS`), you’ll see the dashboard.

> **Note:** Only users listed in `ADMIN_IDS` can access this panel. In SaaS mode, this is typically configured **per tenant**.

## Dashboard Overview
The admin dashboard shows:
* **«📊 Today’s Stats»** — key metrics for today.
* **«🗓️ Today’s Appointments»** — list of confirmed appointments for today.
* **«📈 Weekly Overview»** — trends and busy periods.
* **«⚙️ Settings»** — future features.

## Using the Panel

### Today’s Appointments
1. Tap **«🗓️ Today’s Appointments»**.
2. View the list of clients, times, and services.
3. Next to each appointment is a **«✅ Mark as Done»** button.
4. Tap it when a client arrives or the service is complete.
5. The appointment status changes to **«completed»** and is removed from the active list.

### Statistics
Tap **«📊 Today’s Stats»** to see:
* **Total bookings** for today.
* **Cancellation rate** (percentage of cancelled appointments).
* **Busiest hours** (most appointments per hour).
* **Load by service type** (number of appointments per service).


### Weekly Overview
Tap **«📈 Weekly Overview»** to see:
* Total bookings per day.
* Cancellation trends.
* Busiest days of the week.
* Popular services.

### Marking Appointments as Done
1. In the **«Today’s Appointments»** list, tap **«✅ Mark as Done»**.
2. The appointment status changes to **«completed»**.
3. It’s removed from the active list.
4. The system logs: `Appointment {id} marked done by admin {admin_id}`.

## Metrics Glossary
* **Cancellation Rate:** Percentage of appointments cancelled after booking.
* **Busiest Hours:** Time slots with the highest number of bookings.
* **Load by Service:** Number of appointments per service type.
* **Weekly Trend:** Comparison of bookings across the week.

## Admin Commands
* `/admin` — open the admin dashboard.
* `/stats` — show today’s statistics.
* `/today` — show today’s appointments.

## Troubleshooting
**Issue: I can’t access the admin panel.**  
Check that your Telegram ID is listed in `ADMIN_IDS` (env/config). In SaaS mode, confirm you’re set as an admin for the correct tenant (`client_id`).

**Issue: Stats look incorrect.**  
Verify the DB is up to date and the bot has read permissions.

**Issue: No notifications for new appointments.**  
Ensure `ADMIN_IDS` are correct and the bot is running.


## Contact
For technical issues, contact the dev team:  
* Email: dev@salon.com  
* Telegram: @dev_support_bot
