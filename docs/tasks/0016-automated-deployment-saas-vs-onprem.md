# Task 0016: Automated Deployment (SaaS CI/CD vs On‑Prem Installer)

## Goal
Provide two aligned deployment experiences:

- **SaaS**: automated CI/CD to a hosted cluster
- **On‑Prem**: a simplified installer (script) + Compose workflow for buyers

## Requirements

### SaaS CI/CD (SaaS-only)

* [x] Build and publish container image on every main branch change.
* [x] Run tests and (optionally) migrations in CI.
* [ ] Deploy to the SaaS environment (cluster) with appropriate secrets handling.
* [ ] Include rollback strategy.

### On‑Prem installer (On‑Prem only)

* [x] Provide a `setup` script that:
  * [x] copies `.env.onprem.example` → `.env` (if missing)
  * [x] validates required env vars (`MODE=onprem`, `BOT_TOKEN`, `ADMIN_IDS`, `DATABASE_URL`, `LICENSE_KEY`)
  * [x] boots Compose and prints verification steps
* [x] Keep on‑prem setup offline-friendly where possible.

## Alignment requirements

* [x] `docs/DEPLOY.md` documents both paths clearly.
* [x] `docs/tasks/*` tasks reference the correct path (SaaS-only vs On‑Prem-only).

## Acceptance Criteria

* A SaaS operator can deploy via CI/CD with minimal manual steps.
* An on‑prem buyer can install with a single script + Docker/Compose.

## Implementation note

- Added `scripts/setup-onprem.ps1` for one-command on-prem bootstrap.
- SaaS CI job exists for build/test/publish, while cluster rollout + rollback playbook remains to be finalized.

