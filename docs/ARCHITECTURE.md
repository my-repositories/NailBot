# Architecture Overview

## Components

* **Bot Core** (`src/bot.rs`): Initializes bot, handles updates.
* **Handlers** (`src/handlers/`): Process user/admin commands.
* **FSM** (`src/states.rs`): Manages conversation states.
* **Keyboards** (`src/keyboards/`): Generates inline menus.
* **DB Layer** (`src/database/`): Queries, migrations.
* **Scheduler** (`src/scheduler/`): Reminder tasks.
* **Utils** (`src/utils/`): Helpers (subscription, formatting).

## Data Flow

1. User sends command → Bot receives → Handler processes → Updates FSM → Sends response.
2. Booking: `/start` → Menu → Date → Time → Data → Save → Schedule reminder.
3. Admin: `/admin` → Panel → Manage schedule → Update DB.

## Dependencies

* teloxide → Telegram API
* rusqlite → SQLite
* tokio → Async runtime
* chrono → Date/time
