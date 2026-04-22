# Task 0001: Project Structure Initialization

## Goal
Create the basic project structure and set up the development environment for further development.

## Context
First task in the MVP phase (`ROADMAP.md`). Required before starting any functionality implementation.

## Requirements
* [ ] Create folder structure: `src/`, `migrations/`, `docs/`.
* [ ] Initialize a Cargo project: `cargo init`.
* [ ] Add dependencies to `Cargo.toml` (see `docs/README.md`).
* [ ] Create a basic `src/main.rs` that outputs «Bot started».
* [ ] Set up logging (`docs/LOGGING.md`).

## Technical Details
* Files:
  * `Cargo.toml`
  * `src/main.rs`
* Dependencies: `teloxide`, `tokio`, `rusqlite`, `chrono`, `dotenvy`, `tracing`, `serde`.
* Documentation: `docs/README.md`, `docs/SETUP.md`, `docs/LOGGING.md`.

## Acceptance Criteria
* `cargo build` runs without errors.
* `cargo run` starts the bot and prints a message to the console.
* Logging is working (INFO level).

## Prompt for AI Agent
You are an experienced Rust developer. Create code for task 0001 (project initialization).
1. Write the contents of `Cargo.toml` with the following dependencies: teloxide, tokio, rusqlite, chrono, dotenvy, tracing, serde.
2. Create `src/main.rs`. In it:
   * load configuration from `.env`;
   * set up logging (tracing);
   * implement an async `main()` function that prints «NailBot started» and waits for SIGINT.
