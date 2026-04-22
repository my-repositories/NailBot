# Task 0002: Database and Models Setup

## Goal
Set up an SQLite database, create migrations, and implement Rust models for data operations.

## Context
Second task in the MVP phase (`ROADMAP.md`). Follows project initialization. Forms the foundation for all subsequent features that work with data.


## Requirements
* [ ] Create the `migrations/` folder.
* [ ] Write an SQL migration `001_init.sql` to create the tables (`users`, `appointments`, `time_slots`).
* [ ] Implement Rust structures `User`, `Appointment`, `TimeSlot` in `src/database/models.rs`.
* [ ] Create a module `src/database/mod.rs` with a function `init_db()` for DB connection setup.
* [ ] Test table creation (run the migration).

## Technical Details
* Files:
  * `migrations/001_init.sql`
  * `src/database/models.rs`
  * `src/database/mod.rs`
* Documentation: `docs/DB_SCHEMA.md`, `docs/MODELS.md`.
* Used libraries: `rusqlite`, `serde`, `chrono`.

## Acceptance Criteria
* The `db.sqlite` file is created when the bot starts.
* Tables `users`, `appointments`, `time_slots` are present in the DB.
* Rust structures match the DB schema from `docs/DB_SCHEMA.md`.
* Function `init_db()` successfully connects to the DB and applies the migration.

## Prompt for AI Agent
Based on `docs/DB_SCHEMA.md` and `docs/MODELS.md`, create code for task 0002.
1. Write SQL code for `migrations/001_init.sql`. Create tables `users`, `appointments`, `time_slots` with fields and indexes as described in the documentation.
2. In `src/database/models.rs`, define Rust structures `User`, `Appointment`, `TimeSlot`. Add derive attributes: `Debug`, `Clone`, `Serialize`, `Deserialize`.
3. In `src/database/mod.rs`, implement the function `init_db() -> Result<Connection, Box<dyn std::error::Error>>`. It should:
   * establish a connection to `db.sqlite`;
   * apply the migration from `migrations/001_init.sql`;
   * return a `Connection` object.
