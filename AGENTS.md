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
| **Database backend selection (canonical)** | `docs/tasks/0000-database-choise-and-i18n.md` | PostgreSQL-only baseline and RU/EN i18n strategy |
| **Data models (Rust structs)** | `docs/MODELS.md` | Rules for defining Rust data structures that map to DB tables |
| **FSM states & transitions** | `docs/FSM_GUIDE.md` | State definitions, transition logic, and error handling in conversation flows |
| **Scheduler & reminders** | `docs/SCHEDULER.md` | Setup and management of reminder tasks, time calculations, and cancellation logic |
| **Subscription checks** | `docs/SUBSCRIPTION.md` | Implementing channel membership verification via Telegram API |
| **Error handling & logging** | `docs/LOGGING.md` | Best practices for error propagation, logging levels, and diagnostics |
| **Testing & validation** | `docs/TESTING.md` | Writing unit and integration tests, mocking Telegram API, and data validation |
| **Deployment & operations** | `docs/DEPLOY.md` | Building, containerizing, and running the bot in production |
| **API/Bot service separation** | `docs/tasks/0000-separate-api-service-and-bot-handler.md` | Requirements for splitting Telegram adapter from Web API business core |
| **Hybrid tenant isolation & routing** | `docs/tasks/0014-hybrid-tenant-isolation-and-routing.md` | Hard constraints for SaaS tenant isolation and how updates map to `client_id` |
| **On‑Prem licensing** | `docs/tasks/0015-onprem-license-key-system.md` | License validation rules and on‑prem-only behavior |
| **Automated deployment (SaaS vs On‑Prem)** | `docs/tasks/0016-automated-deployment-saas-vs-onprem.md` | CI/CD for SaaS and installer workflow for on‑prem buyers |
| **Input validation rules** | `docs/VALIDATION_RULES.md` | Canonical validation rules (name/phone/date/time) + tenant-safe callback checks |
| **Centralized error handling** | `docs/ERROR_HANDLING.md` | Error taxonomy, retry rules, logging context (mode/client_id), admin notifications |
| **Reminder system scaling** | `docs/REMINDERS.md` | Reminder models and SaaS scaling constraints (avoid duplicates across workers) |

---

### Quick Reference (Common Scenarios)

* **Need to add a new command?** → Read `docs/HANDLERS_USER.md`.
* **Creating a new DB table?** → Read `docs/DB_SCHEMA.md`.
* **Need DB/backend or translation policy?** → Read `docs/tasks/0000-database-choise-and-i18n.md`.
* **Implementing a user flow with states?** → Read `docs/FSM_GUIDE.md`.
* **Adding a reminder feature?** → Read `docs/SCHEDULER.md`.
* **Checking if user is subscribed?** → Read `docs/SUBSCRIPTION.md`.
* **Setting up the project for the first time?** → Read `docs/SETUP.md`.
* **Need to split bot and backend API?** → Read `docs/tasks/0000-separate-api-service-and-bot-handler.md`.

---

### Notes

* All referenced files are located in the `docs/` directory.
* Always check the **ROADMAP.md** to ensure your task aligns with the current development phase.
* For architectural decisions, consult **ARCHITECTURE.md**.
* If a task isn’t covered here, create a new doc in `docs/` and update this router.
* Hybrid rule of thumb:
  * **SaaS**: every DB access and scheduler job is scoped by `client_id` and must not leak cross-tenant data
  * **On‑Prem**: `client_id` remains constant (usually `1`), and **license checks apply**
* Backend rule of thumb:
  * **PostgreSQL only** for runtime DB
  * **RU/EN i18n via Fluent** with catalogs under `locales/*.ftl`

**Keep it simple. Point, don’t explain.**

