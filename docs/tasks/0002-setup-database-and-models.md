# Task 0002: Database and Models Setup

## Goal
Set up a PostgreSQL database, create migrations, and implement Rust models for data operations.

## Context
Second task in the MVP phase (`ROADMAP.md`). Follows project initialization. Forms the foundation for all subsequent features that work with data.

## API/Bot Split Alignment
* [ ] DB access belongs to API/infrastructure layers only.
* [ ] Bot handlers must consume DB-backed behavior through API contracts, not direct queries.
* [ ] Models/repositories should be documented as backend concerns reusable by Telegram, web, and mobile clients.


## Requirements
* [ ] Create the `migrations/` folder.
* [ ] Write an SQL migration `001_init.sql` to create the tables (`users`, `appointments`, `time_slots`) with **tenant scoping**.
  * [ ] Add `client_id` to all business tables (and `clients` table in SaaS if used)
  * [ ] Add tenant-safe unique constraints (e.g., `UNIQUE (client_id, telegram_id)`; `UNIQUE (client_id, date, time)`)
* [ ] Implement Rust structures `User`, `Appointment`, `TimeSlot` in `src/database/models.rs` including `client_id`.
* [ ] Create a module `src/database/mod.rs` with a function `init_db()` for DB connection setup.
* [ ] Test table creation (run the migration).
* [ ] Tenant isolation rule (must be enforced in all queries):
  * [ ] Every query interface takes `client_id` explicitly (SaaS) or derives it from context (on‑prem constant = 1)

## Technical Details
* Files:
  * `migrations/001_init.sql`
  * `src/database/models.rs`
  * `src/database/mod.rs`
* Documentation: `docs/DB_SCHEMA.md`, `docs/MODELS.md`, `docs/ARCHITECTURE.md`.
* Used libraries: `sqlx`, `serde`, `chrono`.

## Acceptance Criteria
* The PostgreSQL schema is initialized successfully when the API starts.
* Tables `users`, `appointments`, `time_slots` are present in the DB.
* Rust structures match the DB schema from `docs/DB_SCHEMA.md`.
* Function `init_db()` successfully connects to the DB and applies the migration.
* In SaaS mode, it is impossible to read/write a row without providing the correct `client_id` at the repository boundary.

## Prompt for AI Agent
Based on `docs/DB_SCHEMA.md` and `docs/MODELS.md`, create code for task 0002.
1. Write SQL code for `migrations/001_init.sql`. Create tables `users`, `appointments`, `time_slots` with fields and indexes as described in the documentation.
2. In `src/database/models.rs`, define Rust structures `User`, `Appointment`, `TimeSlot` (include `client_id`). Add derive attributes: `Debug`, `Clone`, `Serialize`, `Deserialize`.
3. In `src/database/mod.rs`, implement the function `init_db() -> Result<Connection, Box<dyn std::error::Error>>`. It should:
   * establish a PostgreSQL connection via `DATABASE_URL`;
   * apply the migration from `migrations/001_init.sql`;
   * return a `Connection` object.
