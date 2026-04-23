# Database Schema Guide (Hybrid: SaaS + On‑Prem)

This schema is designed to:

- Support **SaaS multi-tenancy** by scoping all business rows to a `client_id`.
- Remain **seamless for on‑prem single-tenant** deployments by using a single implicit tenant (`client_id = 1`) or by relying on a default value.

## Tenancy Model

### `client_id` rule

- **SaaS**: `client_id` is required and identifies the tenant/workspace/bot instance.
- **On‑Prem**: use one of these conventions (pick one and keep it consistent):
  - **Default tenant**: always write `client_id = 1` and filter by `client_id = 1`
  - **DB default**: define `client_id DEFAULT 1` and still filter by `client_id`

Even in on‑prem, keeping the `client_id` column simplifies portability and avoids “two schemas”.

## Tables

### `clients` (SaaS registry; optional for on‑prem)

- `id` (INTEGER/BIGINT, PK)
- `name` (TEXT, NOT NULL)
- `status` (TEXT, NOT NULL; e.g., `active`, `suspended`)
- `created_at` (DATETIME/TIMESTAMP, DEFAULT now/current_timestamp)

In on‑prem you may keep a single row (`id=1`) or omit this table entirely if you don’t need tenant registry features locally.

## Task alignment

- Tenant routing and isolation requirements: `docs/tasks/0014-hybrid-tenant-isolation-and-routing.md`
- Core DB setup task: `docs/tasks/0002-setup-database-and-models.md`

### `users`

- `id` (INTEGER/BIGINT, PK)
- `client_id` (INTEGER/BIGINT, NOT NULL, FK → `clients.id` in SaaS)
- `telegram_id` (INTEGER/BIGINT, NOT NULL)
- `name` (TEXT)
- `phone` (TEXT)
- `created_at` (DATETIME/TIMESTAMP, DEFAULT now/current_timestamp)

**Uniqueness**:

- SaaS: `UNIQUE (client_id, telegram_id)`
- On‑Prem: still use `UNIQUE (client_id, telegram_id)`; it behaves like a regular unique constraint because `client_id` is constant.

### `appointments`

- `id` (INTEGER/BIGINT, PK)
- `client_id` (INTEGER/BIGINT, NOT NULL)
- `user_id` (INTEGER/BIGINT, NOT NULL, FK → `users.id`)
- `date` (TEXT/DATE, NOT NULL)
- `time` (TEXT/TIME, NOT NULL)
- `status` (TEXT, DEFAULT `'confirmed'`)
- `created_at` (DATETIME/TIMESTAMP, DEFAULT now/current_timestamp)

**Tenant safety**:

- Always filter by `client_id`.
- Recommended invariant: `(appointments.client_id == users.client_id)` enforced in application logic; optionally enforced via composite FKs in Postgres.

### `time_slots`

- `id` (INTEGER/BIGINT, PK)
- `client_id` (INTEGER/BIGINT, NOT NULL)
- `date` (TEXT/DATE, NOT NULL)
- `time` (TEXT/TIME, NOT NULL)
- `is_available` (BOOLEAN, DEFAULT TRUE)
- `created_at` (DATETIME/TIMESTAMP, DEFAULT now/current_timestamp)

**Uniqueness**:

- `UNIQUE (client_id, date, time)`

## Indexes

- `users(client_id, telegram_id)` — fast identity lookup per tenant
- `appointments(client_id, date)` — fast “appointments by day” per tenant
- `appointments(client_id, user_id)` — fast “user appointments” per tenant
- `time_slots(client_id, date, is_available)` — fast availability checks

## Migrations

- Store migrations in `migrations/` (e.g., `migrations/001_init.sql`).
- Apply automatically on startup or via a dedicated migration command in CI/CD.

## SQL Notes (SQLite vs Postgres)

- SQLite lacks some advanced constraints; keep **tenant scoping** enforced in queries and repository methods.
- Postgres can enforce stronger invariants (composite keys, row-level security); consider it for SaaS scale.
