# Task 0003: Implement Basic Bot Handlers

## Goal
Implement basic bot commands: `/start`, `/help`, and the main menu with a keyboard.


## Context
Third task in the MVP phase (`ROADMAP.md`). Follows the database setup. Enables basic user interaction with the bot.

## Requirements
* [ ] Implement a handler for the `/start` command in `src/handlers/user.rs`.
* [ ] Implement a handler for the `/help` command in `src/handlers/user.rs`.
* [ ] Display the main menu with buttons: «Book Appointment», «My Appointments», «Prices».
* [ ] Use the modular keyboard system (`src/keyboards/`).
* [ ] Hybrid routing expectations:
  * [ ] SaaS: handler execution has access to a resolved `client_id`
  * [ ] On‑Prem: `client_id` is constant (e.g., `1`) and still threaded through for consistency


## Technical Details
* Files:
  * `src/handlers/user.rs` — command handlers
  * `src/keyboards/main_menu.rs` — main menu keyboard
  * `src/keyboards/mod.rs` — module root
* Documentation: `docs/ARCHITECTURE.md`, `docs/UI_GUIDE.md`
* Libraries: `teloxide`
* Key considerations:
  * The main menu should be shown both on `/start` and as a fallback option
  * Keyboard should be persistent (not one‑time)
  * Use emoji for visual clarity per `docs/UI_GUIDE.md`
  * In SaaS mode, never show cross-tenant or platform-admin actions in a tenant bot menu (those are SaaS-only)

## Acceptance Criteria
* On `/start`:
  * Bot sends welcome message: «👋 Welcome to NailBot! How can I help you?»
  * Displays main menu keyboard with all buttons
* On `/help`:
  * Sends help message with available commands:
    ```
    📚 Help:

    /start — Show main menu
    /help — Show this help
    /appointment — Book a new appointment

    Use the menu below to navigate:
    ```
  * Attaches the main menu keyboard
* Main menu buttons:
  * «🗓️ Book Appointment» — starts booking flow
  * «👤 My Appointments» — shows user’s appointments
  * «💰 Prices» — displays service prices
  * «📸 Portfolio» — shows example works

* Keyboard is visible and persistent (stays on screen)

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md` and `docs/UI_GUIDE.md`, implement task 0003.

1. In `src/keyboards/`, create the keyboard module structure:
   ```bash
   mkdir src/keyboards
   ```
2. Create `src/keyboards/mod.rs`:
   ```rust
   // File: src/keyboards/mod.rs
   pub mod main_menu;

   // Re‑export for convenience
   pub use main_menu::main_menu;
   ```
3. Create `src/keyboards/main_menu.rs`:
   ```rust
   // File: src/keyboards/main_menu.rs
   use teloxide::types::KeyboardMarkup;

   pub fn main_menu() -> KeyboardMarkup {
       KeyboardMarkup::new(vec![
           vec!["🗓️ Book Appointment".to_string()],
           vec!["👤 My Appointments".to_string(), "💰 Prices".to_string()],
           vec!["📸 Portfolio".to_string()]
       ])
       .resize_keyboard(true)  // Make buttons smaller
   }
   ```
4. In `src/handlers/user.rs`, implement handlers:
   * For `/start`:
     * Send welcome message
     * Attach `keyboards::mainmenu::mainmenu()` keyboard
   * For `/help`:
     * Send help text
     * Attach main menu keyboard
5. Ensure both handlers:
   * Are registered in the bot’s dispatcher
   * Use `send_message()` with `reply_markup` parameter
   * Log the command execution via `tracing::info!`
