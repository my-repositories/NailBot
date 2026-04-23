# Setup Guide (Hybrid: SaaS + On‑Prem)

This project supports two operating modes:

- **SaaS (Multi-tenant)**: one hosted instance serves multiple clients/tenants.
- **On‑Prem (Single-tenant)**: a customer-hosted instance serves a single tenant and requires a license key.

## Prerequisites

- Rust toolchain (MSRV as defined by the repo; typically 1.70+ for modern async stacks)
- A database:
  - SQLite (simple, good for on‑prem and small installs)
  - PostgreSQL (recommended for SaaS production)
- Telegram bot token(s) from @BotFather

## Environment Strategy

All configuration is grouped into three layers of environment variables.

### 1) Global (applies to both SaaS and On‑Prem)

- `MODE`: `saas` or `onprem`
- `DATABASE_URL`: e.g. `sqlite:///app/data/bot.db` or `postgres://user:pass@db:5432/nailbot`
- `LOG_LEVEL`: e.g. `info`, `debug`
- `TZ`: e.g. `Europe/Moscow`

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

3. Edit `.env` (at minimum set `MODE=onprem`, `BOT_TOKEN`, `ADMIN_IDS`, and `DATABASE_URL`).

4. Build and run:

```bash
cargo build
cargo run
```

## SaaS Mode Setup (Conceptual)

SaaS mode requires a tenant registry and a way to route updates per tenant/bot.

- If you use **polling**: typically run one polling worker per tenant token (horizontal scaling).
- If you use **webhooks**: route by URL path, host header, or bot-specific webhook secret per tenant.

The core business logic remains the same; only the configuration layer and update routing differ.

## Troubleshooting

- **No such file or directory**: ensure your env file exists and is loaded (or variables are set in the shell/container).
- **Invalid token**: verify bot token format from @BotFather.
- **Database locked (SQLite)**: ensure only one writer process is using the DB file; for multi-worker workloads prefer Postgres.
