# AGENTS.md (Router Edition)

**Purpose:** This file acts as a router for AI agents. It tells which documentation file to read for a specific task — no code, no long explanations.


### How to use

1. Identify your task (e.g., «create a handler», «write a DB migration»).
2. Find the matching category below.
3. Read the referenced file for detailed instructions and prompts.
4. Generate code following the guidelines in that file.

### Task → Documentation Mapping

| Task Category | Read This File | Purpose of the Referenced File |
|-------------|---------------|-------------------------------|
| **Bot setup & configuration** | `docs/SETUP.md` | Guides on project initialization, environment setup, and bot configuration |
| **User interface & keyboards** | `docs/UI_GUIDE.md` | Instructions for creating inline keyboards, menus, and user‑facing messages |
| **Command handlers (user side)** | `docs/HANDLERS_USER.md` | Prompts for implementing user commands (`/start`, appointment flow, etc.) |
| **Admin panel features** | `docs/ADMIN_PANEL.md` | Guidelines for admin commands, schedule management, and moderation tools |
| **Database schema & migrations** | `docs/DB_SCHEMA.md` | SQL templates, table definitions, migration scripts, and model structures |
| **Data models (Rust structs)** | `docs/MODELS.md` | Rules for defining Rust data structures that map to DB tables |
| **FSM states & transitions** | `docs/FSM_GUIDE.md` | State definitions, transition logic, and error handling in conversation flows |
| **Scheduler & reminders** | `docs/SCHEDULER.md` | Setup and management of reminder tasks, time calculations, and cancellation logic |
| **Subscription checks** | `docs/SUBSCRIPTION.md` | Implementing channel membership verification via Telegram API |
| **Error handling & logging** | `docs/LOGGING.md` | Best practices for error propagation, logging levels, and diagnostics |
| **Testing & validation** | `docs/TESTING.md` | Writing unit and integration tests, mocking Telegram API, and data validation |
| **Deployment & operations** | `docs/DEPLOY.md` | Building, containerizing, and running the bot in production |

---

### Quick Reference (Common Scenarios)

* **Need to add a new command?** → Read `docs/HANDLERS_USER.md`.
* **Creating a new DB table?** → Read `docs/DB_SCHEMA.md`.
* **Implementing a user flow with states?** → Read `docs/FSM_GUIDE.md`.
* **Adding a reminder feature?** → Read `docs/SCHEDULER.md`.
* **Checking if user is subscribed?** → Read `docs/SUBSCRIPTION.md`.
* **Setting up the project for the first time?** → Read `docs/SETUP.md`.

---

### Notes

* All referenced files are located in the `docs/` directory.
* Always check the **ROADMAP.md** to ensure your task aligns with the current development phase.
* For architectural decisions, consult **ARCHITECTURE.md**.
* If a task isn’t covered here, create a new doc in `docs/` and update this router.

**Keep it simple. Point, don’t explain.**

