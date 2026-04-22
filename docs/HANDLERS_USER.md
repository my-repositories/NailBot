# User Handlers Guide

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

## Prompts for AI

> Implement `/start` handler in `src/handlers/user.rs`. Show main menu with inline buttons: «Book», «My Appointments», «Prices», «Portfolio». Use `teloxide::KeyboardMarkup`. Log user ID on start.

> Create `handle_appointment_flow` function. Start FSM state `SelectingDate`. Send calendar keyboard from `src/keyboards/calendar.rs`.
