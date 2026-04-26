# Task 0001: Project Structure Initialization

## Goal
Create the basic project structure and set up the development environment for further development.

## Context
First task in the MVP phase (`ROADMAP.md`). Required before starting any functionality implementation.

## API/Bot Split Alignment
* [ ] Define initial structure for separated services:
  * [ ] `src/api/` (HTTP surface)
  * [ ] `src/application/` and `src/domain/` (business rules)
  * [ ] `src/bot/` (Telegram adapter only)
* [ ] Keep all new documentation/examples aligned with `docs/tasks/0000-separate-api-service-and-bot-handler.md`.

## Requirements
* [ ] Create folder structure: `src/`, `migrations/`, `docs/`.
* [ ] Initialize a Cargo project: `cargo init`.
* [ ] Add dependencies to `Cargo.toml` (see `docs/README.md`).
* [ ] Create a basic `src/main.rs` that outputs «Bot started».
* [ ] Set up logging (`docs/LOGGING.md`).
* [ ] Define distribution mode wiring early:
  * [ ] `MODE=saas|onprem` parsed into a typed `Settings` value
  * [ ] a `TenantContext` concept exists even in on‑prem (constant `client_id = 1`)

## Technical Details
* Files:
  * `Cargo.toml`
  * `src/main.rs`
* Dependencies: `teloxide`, `tokio`, `sqlx`, `chrono`, `dotenvy`, `tracing`, `serde`.
* Documentation: `docs/README.md`, `docs/SETUP.md`, `docs/LOGGING.md`, `docs/ARCHITECTURE.md`.

## Acceptance Criteria
* `cargo build` runs without errors.
* `cargo run` starts the bot and prints a message to the console.
* Logging is working (INFO level).

## Prompt for AI Agent
You are an experienced Rust developer. Create code for task 0001 (project initialization).
1. Write the contents of `Cargo.toml` with the following dependencies: teloxide, tokio, sqlx, chrono, dotenvy, tracing, serde.
2. Create `src/main.rs`. In it:
   * load configuration from `.env`;
   * set up logging (tracing);
   * implement an async `main()` function that prints «NailBot started» and waits for SIGINT.
3. Ensure `MODE=saas|onprem` is parsed and logged at startup (without leaking secrets).
