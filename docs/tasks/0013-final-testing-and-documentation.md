# Task 0013: Final Testing & Documentation


## Goal
Ensure the entire booking flow is stable, bug‑free, and well‑documented for users and administrators.


## Context
Thirteenth and final task in the MVP phase (`ROADMAP.md`). Follows error handling implementation. Completes the MVP by validating all functionality and preparing user‑facing materials.


## Requirements
* [ ] Write integration tests for the full booking flow
* [ ] Test edge cases (invalid input, time conflicts, timezone issues)
* [ ] Verify all error handling scenarios
* [ ] Update user and admin documentation
* [ ] Prepare deployment guide
* [ ] Perform UX walkthrough
* [ ] Create release checklist

## Technical Details
* Files:
  * `tests/integration/booking_flow.rs` (new — integration tests)
  * `docs/USER_GUIDE.md` (update — user guide)
  * `docs/ADMIN_GUIDE.md` (update — admin guide)
  * `docs/DEPLOY.md` (update — deployment instructions)
  * `README.md` (final update)
* Documentation: `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, `docs/UI_GUIDE.md`
* Libraries: `teloxide`, `tokio`, `rstest` (for testing), `tracing`
* Key considerations:
  * Test both happy path and error scenarios
  * Use mock Telegram API for integration tests
  * Verify timezone handling (UTC vs local)
  * Ensure all user messages are clear and consistent
  * Check keyboard navigation flows

## Acceptance Criteria
* Integration tests:
  * Cover full flow: `/start` → date selection → time selection → data input → confirmation
  * Test reminders (24 h and 1 h)
  * Test `/myappointments` with cancel/reschedule
  * Test admin notifications
  * All tests pass with 100 % coverage of core handlers
* Edge cases:
  * Invalid date/time input handled gracefully
  * Double booking prevented
  * Timezone conversion correct (UTC+3)
  * Session timeout handled
* Documentation:
  * `USER_GUIDE.md`: clear instructions for booking, viewing, cancelling appointments
  * `ADMIN_GUIDE.md`: admin commands, stats interpretation, action logs
  * `DEPLOY.md`: step‑by‑step setup (DB, config, environment variables)
  * `README.md`: project overview, quick start, tech stack
* UX:
  * Flow feels smooth and intuitive
  * Messages are concise and helpful
  * Key actions have clear feedback (success/error)
* Release checklist:
  * All tasks 0001–0012 marked as done
  * Code reviewed and merged to `main`
  * Tests passing in CI/CD
  * Docs up to date
  * Env templates included (`.env.onprem.example`, `.env.saas.example`)

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md`, `docs/DB_SCHEMA.md`, and `docs/UI_GUIDE.md`, implement task 0013.

1. Create `tests/integration/booking_flow.rs`:
   ```rust
   // File: tests/integration/booking_flow.rs
   use rstest::rstest;
   use teloxide::Bot;

   #[rstest]
   #[tokio::test]
   async fn test_full_booking_flow() {
       // Arrange: mock bot, DB, user session
       // Act: simulate user steps:
       //   /start → select date → select time → input name/phone → confirm
       // Assert: appointment saved in DB, reminder scheduled, confirmation message sent
   }

   #[rstest]
   #[tokio::test]
   async fn test_cancel_from_reminder() {
       // Arrange: confirmed appointment nearing reminder time
       // Act: send "Cancel" from reminder keyboard
       // Assert: status updated to 'cancelled', reminder cancelled
   }
   ```
2. Update `docs/USER_GUIDE.md`:
   * Add step‑by‑step booking instructions
   * Explain `/myappointments` usage
   * Include screenshots of key screens (if available)
3. Update `docs/ADMIN_GUIDE.md`:
   * List admin commands (`/admin`, `/stats`)
   * Describe dashboard buttons and their effects
   * Explain stats metrics (cancellation rate, busiest hours)
4. Create `docs/DEPLOY.md`:
   * Prerequisites: Docker/Compose (recommended), or Rust + DB + environment variables
   * Steps:
     1. Copy `.env.onprem.example` or `.env.saas.example` to `.env`
     2. Edit `.env` (token(s), admin IDs, DB URL, mode)
     3. Run via `docker compose up -d --build` (or build native)
   * Environment variables: see `docs/SETUP.md`
5. Update `README.md`:
   * Project title and description
   * Features (appointment booking, reminders, admin panel)
   * Quick start section
   * Tech stack: Rust, teloxide, postgres, chrono
   * Contributing guidelines
6. Perform manual UX walkthrough:
   * Test all keyboard flows
   * Verify message clarity
   * Check error messages
   * Ensure state transitions are smooth
7. Generate release checklist:
   * Confirm all tasks 0001–0012 are complete
   * Run `cargo test --all`
   * Push final docs to repo
   * Tag release `v1.0.0-mvp`

