# Database Schema Guide

## Tables

### `users`

* `id` (INTEGER, PK, AUTOINCREMENT)
* `telegram_id` (INTEGER, UNIQUE, NOT NULL)
* `name` (TEXT)
* `phone` (TEXT)
* `created_at` (DATETIME, DEFAULT CURRENT_TIMESTAMP)

### `appointments`

* `id` (INTEGER, PK, AUTOINCREMENT)
* `user_id` (INTEGER, FK → users.id, NOT NULL)
* `date` (TEXT, YYYY-MM-DD, NOT NULL)
* `time` (TEXT, HH:MM, NOT NULL)
* `status` (TEXT, DEFAULT 'confirmed')
* `created_at` (DATETIME, DEFAULT CURRENT_TIMESTAMP)

### `time_slots`

* `id` (INTEGER, PK, AUTOINCREMENT)
* `date` (TEXT, YYYY-MM-DD, NOT NULL)
* `time` (TEXT, HH:MM, NOT NULL)
* `is_available` (BOOLEAN, DEFAULT TRUE)
* `created_at` (DATETIME, DEFAULT CURRENT_TIMESTAMP)

## Indexes

* `appointments(date)` — for fast date lookup.
* `time_slots(date, is_available)` — for availability checks.
* `users(telegram_id)` — unique index.

## Migrations

* File: `migrations/001_init.sql`.
* Apply automatically on startup.

## SQL Prompts

> Create migration `migrations/001_init.sql`. Define tables `users`, `appointments`, `time_slots`. Add primary keys, foreign keys, and indexes. Use SQLite syntax.
