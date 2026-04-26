# Deployment Guide (Universal Docker Workflow)

This repository supports a **hybrid distribution model**:

- **SaaS (Multi-tenant)**: you operate one hosted platform instance.
- **On‑Prem (Single-tenant)**: a customer runs one instance with PostgreSQL and must provide a valid license key.

The deployment workflow below uses **Docker + Compose** so both distributions share the same runtime story.

## Prerequisites

- Docker Engine + Docker Compose v2
- Telegram bot token(s)
- PostgreSQL 15+

## Container Images (Target Layout)

- `api` container: HTTP API service with domain logic and DB access.
- `bot` container: Telegram adapter calling `api` endpoints.
- `db` container: PostgreSQL for SaaS and On‑Prem.

## Environment Files

Use one of the provided templates:

- `.env.onprem.example` → copy to `.env` for single-tenant installs
- `.env.saas.example` → copy to `.env` for hosted multi-tenant installs

See `docs/SETUP.md` for the env variable taxonomy (Global vs SaaS vs On‑Prem).

## On‑Premise Deployment (Single-tenant)

1. Copy env template:

```bash
cp .env.onprem.example .env
```

2. Edit `.env` and set at least:

- `MODE=onprem`
- `BOT_TOKEN=...`
- `ADMIN_IDS=...`
- `DATABASE_URL=postgres://nailbot:nailbot@db:5432/nailbot`
- `LICENSE_KEY=...`

3. Start:

```bash
docker compose --env-file .env up -d --build
```

4. Verify:

- Send `/start` to the bot.
- Check API and bot logs:

```bash
docker compose logs -f api bot
```

## SaaS Deployment (Multi-tenant)

SaaS mode requires a tenant registry (DB-backed, file-backed, or env-backed) and may require Postgres.

1. Copy env template:

```bash
cp .env.saas.example .env
```

2. Edit `.env`:

- `MODE=saas`
- `DATABASE_URL=postgres://nailbot:nailbot@db:5432/nailbot`
- `SAAS_TENANT_REGISTRY=...`
- Optional billing variables if enabled

3. Start (API + bot + optional Postgres):

```bash
docker compose --env-file .env --profile postgres up -d --build
```

## Automated deployment alignment (Tasks)

- **SaaS CI/CD**: see `docs/tasks/0016-automated-deployment-saas-vs-onprem.md` (SaaS-only).
- **On‑Prem installer**: see `docs/tasks/0016-automated-deployment-saas-vs-onprem.md` (On‑Prem only).
- **API/Bot separation baseline**: see `docs/tasks/0000-separate-api-service-and-bot-handler.md`.

## License Key Verification (On‑Prem Concept)

On‑prem deployments should include a license check to reduce unauthorized redistribution. The recommended approach is **defense-in-depth**:

### Verification points

- **Startup**: fail fast if the license is missing/invalid.
- **Periodic re-check**: e.g., every 24 hours (with backoff if the license server is down).
- **Feature gating** (optional): allow read-only features when license is expired, but block booking/admin actions.

### Verification models

- **Offline** (recommended for air‑gapped customers):
  - License is a signed token (e.g., Ed25519) containing `customer_id`, `expires_at`, and optional `fingerprint`.
  - App verifies signature using an embedded public key; no network needed.
- **Online**:
  - App calls `LICENSE_SERVER_URL` with `LICENSE_KEY` and instance fingerprint; caches the result with TTL.

### Instance fingerprinting (optional)

If you choose to bind a license to an installation, fingerprint from stable inputs such as:

- customer-provided `INSTANCE_ID`
- (optional) machine identifiers (be cautious: may break container portability)

### Security notes

- Assume the customer can patch binaries (on‑prem threat model). Licensing reduces casual redistribution; it does not prevent a determined reverse engineer.
- Never log full tokens/keys. Redact or hash.

## Localization deployment notes (RU/EN)

- Ship `locales/en.ftl` and `locales/ru.ftl` with both API and bot images.
- Keep locale selection in runtime config and tenant/user profile, never in code branches.
- Validate that API and bot containers run the same translation catalogs and key versions.

## Tenant isolation note (SaaS)

SaaS deployments must ensure **every request and background job** is executed with a resolved `client_id` and uses tenant-scoped queries. See `docs/tasks/0014-hybrid-tenant-isolation-and-routing.md`.

In split deployments, this applies to both:

- API request handling
- bot-to-API context propagation

## Operational Notes

- **Backups**:
  - Postgres: use `pg_dump` or physical backups.

## Troubleshooting

- **Bot won’t start**: verify env vars are set; inspect `docker compose logs -f app`.
- **Database errors**: check `DATABASE_URL`, file permissions (SQLite), or DB connectivity (Postgres).
- **No admin access**: confirm `ADMIN_IDS` format (comma-separated integers).
