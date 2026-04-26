# Setup Guide (Hybrid: SaaS + On‑Prem)

This project supports two operating modes:

- **SaaS (Multi-tenant)**: one hosted instance serves multiple clients/tenants.
- **On‑Prem (Single-tenant)**: a customer-hosted instance serves a single tenant and requires a license key.

Service topology target:

- **API service**: business logic + DB access
- **Bot service**: Telegram adapter that calls API

## Prerequisites

- Rust toolchain (MSRV as defined by the repo; typically 1.70+ for modern async stacks)
- PostgreSQL 15+ (single required runtime database for all modes)
- Telegram bot token(s) from @BotFather

## Environment Strategy

All configuration is grouped into three layers of environment variables.

### 1) Global (applies to both SaaS and On‑Prem)

- `MODE`: `saas` or `onprem`
- `DATABASE_URL`: `postgres://user:pass@db:5432/nailbot`
- `LOG_LEVEL`: e.g. `info`, `debug`
- `TZ`: e.g. `Europe/Moscow`
- `API_BASE_URL`: base URL for clients (bot/web/mobile) to reach Web API
- `API_TIMEOUT_MS`: request timeout for internal clients (for example, bot -> api)

### 2) SaaS-specific (hosted, multi-tenant)

- `SAAS_MASTER_BOT_TOKEN`: a “control plane” bot token (optional) used for admin/billing/support workflows
- `SAAS_TENANT_REGISTRY`: where tenants/tokens come from (conceptual; implement one approach)
  - Example values: `db`, `file:/app/config/tenants.json`, `env`
- `BILLING_PROVIDER`: `stripe` / `manual` / `none`
- `BILLING_WEBHOOK_SECRET`: if using Stripe/webhooks

**Tenant bot tokens** (choose one strategy; don’t mix):

- **DB-backed**: store encrypted tokens in the SaaS DB
- **File-backed**: `tenants.json` mounted into the container
- **Env-backed**: `TENANT_<CLIENT_ID>_BOT_TOKEN` entries (works for small N)

### 3) Local/On‑Prem specific (single-tenant)

- `BOT_TOKEN`: the single tenant bot token
- `ADMIN_IDS`: comma-separated Telegram IDs (e.g. `123,456`)
- `CHANNEL_ID`: channel/group ID for subscription checks (optional)
- `CHANNEL_LINK`: public invite link (optional)
- `LICENSE_KEY`: customer license key (required in on‑prem mode)
- `LICENSE_SERVER_URL`: optional, if validating online

## Quick Start (Development)

1. Clone the repository:

```bash
git clone <url>
cd NailBot
```

2. Create a local env file:

```bash
cp .env.onprem.example .env
```

3. Edit `.env` (at minimum set `MODE=onprem`, `BOT_TOKEN`, `ADMIN_IDS`, `DATABASE_URL`, `DEFAULT_LOCALE`, and `LICENSE_KEY`).

4. Run strategy (documentation target):

- **Monolith dev mode**: bot and API in one process for faster local iteration.
- **Split mode**: run API and bot as separate processes/services to match production.

5. Build and run:

```bash
cargo build
cargo run
```

## Localization baseline (RU/EN)

- Translation engine: `fluent` + `fluent-bundle`.
- Locale IDs and fallback: `unic-langid`.
- Catalog files are shared across channels:
  - `locales/en.ftl`
  - `locales/ru.ftl`
- SaaS locale resolution: user locale is loaded from tenant-scoped DB data per request/session.
- On-Prem locale resolution: default from `.env` (`DEFAULT_LOCALE`) with optional per-user override.
- Bot/API rule: business logic stays language-neutral; adapters render localized text by key.

## SaaS Mode Setup (Conceptual)

SaaS mode requires a tenant registry and a way to route updates per tenant/bot.

- If you use **polling**: typically run one polling worker per tenant token (horizontal scaling).
- If you use **webhooks**: route by URL path, host header, or bot-specific webhook secret per tenant.

The core business logic remains the same; only the configuration layer and update routing differ.

## Separation task reference

For full requirements of the API/bot split, see:

- `docs/tasks/0000-separate-api-service-and-bot-handler.md`

## Troubleshooting

- **No such file or directory**: ensure your env file exists and is loaded (or variables are set in the shell/container).
- **Invalid token**: verify bot token format from @BotFather.
- **Database connection/contention errors**: verify `DATABASE_URL`, DB availability, and retry/backoff settings.
