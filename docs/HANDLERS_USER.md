# User Handlers Guide

This guide must be implemented in a **tenant-aware** way:

- **SaaS (multi-tenant)**: every handler runs with a resolved `client_id` (derived from which bot token/webhook received the update, or from a routing layer). All DB reads/writes must be scoped to that `client_id`.
- **On‑Prem (single-tenant)**: `client_id` is constant (commonly `1`) and can be injected by config.

## Commands

* `/start` — Show main menu.
* `/appointment` — Start booking flow.
* `/my_appointments` — List user's appointments.
* `/prices` — Show price list.
* `/portfolio` — Show portfolio.

## Handler Rules

* All handlers are `async fn`.
* Return `Result<(), Box<dyn std::error::Error>>`.
* Use FSM states from `src/states.rs`.
* Log key actions with `tracing`.
* In SaaS mode, include `client_id` in logs and propagate it into all repository/query calls.

## Prompts for AI

> Implement `/start` handler in `src/handlers/user.rs`. Show main menu with inline buttons: «Book», «My Appointments», «Prices», «Portfolio». Use `teloxide::KeyboardMarkup`. Log user ID and `client_id` (SaaS).

> Create `handle_appointment_flow` function. Start FSM state `SelectingDate`. Send calendar keyboard from `src/keyboards/calendar.rs`. Ensure the session key is tenant-aware (SaaS: `(client_id, telegram_id)`).
