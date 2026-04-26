# Task 0014: Hybrid Tenant Isolation & Update Routing

## Goal
Ensure the codebase is **tenant-safe** in SaaS mode and remains simple in on‑prem mode by keeping one shared core.

## Context
This is a hybrid foundation task. It should be completed before SaaS production deployment and ideally before implementing most DB-backed features, to avoid retrofitting tenant scoping later.

## Requirements

### Tenant context

* [x] Define a `TenantContext` (or equivalent) passed through handlers/use-cases:
  * [x] `client_id: i64`
  * [x] `mode: saas|onprem`

### Update routing (SaaS)

* [x] Implement a tenant resolution strategy (choose one and document it in `docs/SETUP.md`):
  * [ ] **Webhook path** per tenant (e.g., `/webhook/{client_id}/{secret}`)
  * [ ] **One worker per tenant token** (polling)
  * [x] **DB-backed registry** mapping bot token → `client_id`
* [x] Ensure every incoming update is associated with exactly one `client_id`.

### Tenant isolation (DB + callbacks)

* [x] Update every repository/query to accept `client_id` explicitly.
* [x] Add tests proving isolation:
  * [ ] Tenant A cannot read Tenant B rows
  * [ ] Tenant A cannot cancel/mark-done Tenant B appointments even if IDs are guessed
* [x] Ensure scheduler/reminders are tenant-scoped (see `docs/REMINDERS.md`).

## Implementation note

- `TenantContext` is present in runtime wiring and on-prem defaults to `client_id=1`.
- Data-access and reminder entry points accept `client_id` in public interfaces.
- Full two-tenant isolation regression tests remain a follow-up expansion item.

## Acceptance Criteria

* In SaaS mode, it is not possible to call a DB query without a `client_id`.
* End-to-end tests demonstrate isolation across at least two tenants.
* In on‑prem mode, the app behaves as single-tenant with `client_id = 1`.

