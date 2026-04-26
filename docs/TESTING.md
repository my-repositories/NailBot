# Testing Guide

Hybrid requirement:

- Run the core test suite in **both modes**:
  - **On‑Prem**: single tenant (`client_id = 1`), license required
  - **SaaS**: at least two tenants (`client_id = A`, `client_id = B`) to verify isolation

## Types

* **Unit Tests:** Individual functions (e.g., `check_subscription`, `parse_date`).
* **Integration Tests:** Command handlers with mock bot/DB.
* **End‑to‑End (E2E):** Full booking flow simulation.

## Tools

* `cargo test` — run all tests.
* `mockall` — mock external dependencies (Telegram API).
* `testcontainers` / dedicated Postgres test DB — run integration tests against PostgreSQL.

## Test Structure

* Place tests in `tests/` directory.
* Unit tests: `src/*/mod.rs` → `#[cfg(test)]` modules.
* Integration tests: `tests/integration/`.

## Examples

### Unit Test (subscription)
```rust
#[test]
fn test_check_subscription_member() {
    // Mock bot returns Member status
    let result = check_subscription(12345).unwrap();
    assert_eq!(result, true);
}
```

### Integration Test (booking handler)
```rust
#[tokio::test]
async fn test_handle_appointment_flow() {
    // Mock bot, DB, FSM
    // Simulate: /appointment → select date → select time → input data
    // Assert: appointment saved in DB
}
```

## AI Prompts

> Write unit test for `check_subscription` in `src/utils/subscription.rs`. Mock `Bot::get_chat_member` to return `Member`. Assert result is `true`. Add test for `Left` status → `false`.

> Create integration test for `/start` handler. Mock bot updates. Verify main menu is sent. Use `assert_eq!` for message text.

## Tenant isolation test cases (SaaS)

- Create user/appointments under tenant A; ensure tenant B cannot list/cancel/modify them.
- Ensure scheduler/reminder scans don’t cross tenants.
