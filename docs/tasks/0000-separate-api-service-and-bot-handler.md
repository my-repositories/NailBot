# Task 0000: Separate Web API Service and Bot Handler

## Goal
Refactor the application into two independent layers:

1. **Web API service** that owns business logic and data access
2. **Telegram bot handler** that only handles Telegram transport and calls the Web API

This separation enables reuse of the same backend for future **web** and **mobile** clients.

## Context
The current bot-centric architecture is hard to extend to non-Telegram clients. We need a transport-agnostic backend contract so new channels (web/mobile) can use the same booking domain without duplicating logic.

## Requirements

### Service boundaries

* [ ] Define clear module/service boundaries:
  * [ ] `api` (HTTP controllers, request/response DTOs)
  * [ ] `application` (use-cases, validation rules, orchestration)
  * [ ] `domain` (entities/value objects/business rules)
  * [ ] `infrastructure` (DB repositories, external adapters)
  * [ ] `bot` (Telegram update parsing + API client calls only)
* [ ] Bot layer must not access repositories or DB directly.
* [ ] Business logic must be moved out of Telegram handlers into application/domain layers exposed by API endpoints.

### API contract for multi-client use

* [ ] Introduce versioned HTTP endpoints (for example: `/api/v1/...`) for core flows:
  * [ ] availability calendar/time slots
  * [ ] appointment creation
  * [ ] appointment listing/cancellation
  * [ ] admin schedule operations
* [ ] Define stable request/response DTOs and error format suitable for Telegram, web, and mobile clients.
* [ ] Keep validation centralized in API/application layer, not duplicated in bot handlers.

### Tenant safety and deployment mode compatibility

* [ ] Preserve hybrid behavior:
  * [ ] SaaS: every API request and DB operation is scoped by `client_id`
  * [ ] On-prem: single tenant behavior (`client_id = 1`) with license checks unchanged
* [ ] Ensure bot-to-API calls always include tenant context required by `docs/tasks/0014-hybrid-tenant-isolation-and-routing.md`.

### Internal API client for bot

* [ ] Implement a typed bot API client module (retry + timeout + structured error mapping).
* [ ] Map API errors to user-friendly Telegram messages without leaking internals.
* [ ] Keep bot state/FSM focused on dialog flow only; business decisions come from API responses.

### Migration and compatibility

* [ ] Introduce the split incrementally with feature parity for existing Telegram flows.
* [ ] Document run modes in `README.md` and `docs/SETUP.md`:
  * [ ] monolith process (bot + api) for local/dev
  * [ ] separated processes/services for production scaling
* [ ] Add transitional feature flags if needed to reduce rollout risk.

## Acceptance Criteria

* Telegram bot can run without direct DB access and still complete all current core booking flows through Web API.
* At least one endpoint set (booking flow) is consumed by bot via typed HTTP client end-to-end.
* API contracts are documented and reusable for non-Telegram clients (web/mobile).
* SaaS tenant isolation and on-prem license behavior remain valid after refactor.
* Tests cover:
  * [ ] API use-case behavior
  * [ ] bot-to-api integration
  * [ ] regression for existing Telegram scenarios