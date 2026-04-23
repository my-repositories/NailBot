# Task 0015: On‑Prem License Key System

## Goal
Implement a license key system for **on‑prem** installs: generation format (concept), validation, and expiration handling, with safe logs and operator-friendly behavior.

## Context
On‑prem buyers run a single-tenant instance and must provide a valid license key. This is **on‑prem-only** and must not block SaaS runtime paths.

## Requirements

### License validation points

* [ ] Validate at startup (fail fast or enter restricted mode).
* [ ] Periodically re-check (e.g., every 24 hours) with backoff on failures.
* [ ] Never log the full license key (redact/hash).

### Verification model (choose one; document in `docs/DEPLOY.md`)

* [ ] **Offline signed token** (recommended):
  * [ ] License is a signed payload (e.g., Ed25519) containing `customer_id`, `expires_at`, optional `features`.
  * [ ] App embeds public key and verifies signature locally.
* [ ] **Online verification** (optional):
  * [ ] App calls `LICENSE_SERVER_URL` with `LICENSE_KEY` and optional fingerprint.
  * [ ] Cache verification result with TTL.

### Behavior when invalid/expired

* [ ] Define behavior:
  * [ ] Hard stop at boot (strict)
  * [ ] Or allow read-only / limited mode (graceful) and block booking/admin actions
* [ ] User-facing messaging is friendly; admin notification includes next steps.

## Acceptance Criteria

* On‑prem mode refuses to run core flows without a valid license (per chosen policy).
* SaaS mode ignores license settings entirely.
* Tests cover valid, expired, invalid signature, and network-down (if online) cases.

