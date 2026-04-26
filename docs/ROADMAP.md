# Roadmap for NailBot

## MVP (v0.1) — 2–3 Weeks

**Goal:** Establish API-first backend and keep Telegram as first client.

* Epic 0: API/Bot separation foundation (`docs/tasks/0000-separate-api-service-and-bot-handler.md`)
* Epic 1: Core Infrastructure
* Epic 2: Database
* Epic 3: User Booking Flow (Telegram via API)
* Epic 4: Admin Panel (Telegram via API)
* Epic H1: Hybrid foundations (MODE routing, tenant context, on‑prem license gate)
  * Tenant isolation checklist for every repository/query
  * License key system (on‑prem): validate at startup + periodic re-check
  * Deployment split: SaaS CI/CD vs on‑prem installer script

## Phase 2 (v0.2) — 1–2 Weeks

**Goal:** Improve UX and add automation.

* Epic 5: Notifications
* Epic 3 Enhancements: Cancellation, subscription check, prices/portfolio
* Epic H2: SaaS operational hardening (SaaS-only)
  * Multi-tenant admin dashboard (platform)
  * Tenant lifecycle management
  * Billing/entitlements (if enabled)

## Phase 3 (v1.0) — 1 Week

**Goal:** Stabilization and polishing.

* Epic 6: Additional Features (logging, metrics, docs, deployment)

## Future (v1.x)

* Web app client (consumes same API)
* Mobile app client (consumes same API)
* Online payment
* Review system
* Admin stats
* Multi‑language
* Telegram Mini App (as additional API client)
* DB backup
* SaaS-only: metering, per-tenant quotas, audit logs
