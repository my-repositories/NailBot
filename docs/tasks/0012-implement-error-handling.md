# Task 0012: Implement Error Handling & Graceful Degradation


## Goal
Ensure the bot remains operational and provides meaningful feedback during errors (network, DB, API failures). Implement graceful degradation and recovery mechanisms.

## Context
Twelfth task in the MVP phase (`ROADMAP.md`). Follows admin panel implementation. Critical for production readiness — prevents crashes and improves user experience during transient failures.

## Requirements
* [ ] Implement centralized error types in `src/error.rs`
* [ ] Add retry logic for transient failures (DB, Telegram API)
* [ ] Create fallback responses when services are unavailable
* [ ] Ensure user sessions persist across errors
* [ ] Log errors with context (user ID, state, operation)
* [ ] Notify admins of critical errors
* [ ] Display user‑friendly messages instead of stack traces
* [ ] Test error scenarios in handlers

## Technical Details
* Files:
  * `src/error.rs` (new — custom error types)
  * `src/utils/retry.rs` (new — retry policy)
  * `src/handlers/mod.rs` (extend error handling)
  * `src/main.rs` (global error middleware)
* Documentation: `docs/ERROR_HANDLING.md`, `docs/ARCHITECTURE.md`, `docs/LOGGING.md`
* Libraries: `teloxide`, `anyhow`, `tokio`, `tracing`
* Key considerations:
  * Use `Result<T, AppError>` consistently
  * Implement exponential backoff for retries (max 3 attempts)
  * Distinguish transient vs. permanent errors
  * Redact PII in logs
  * Avoid infinite retry loops
  * Ensure FSM state is preserved during errors
  * Hybrid:
    * SaaS: error logs and notifications must include `client_id` and must not leak other tenants' data
    * On‑Prem: license errors are treated as non-recoverable or feature-gating (per `docs/DEPLOY.md`)

## Acceptance Criteria
* All handlers:
  * Return `Result<(), AppError>` instead of panicking
  * Catch and handle specific errors (DB timeout, network error, etc.)
* On transient errors:
  * Retry operation with exponential backoff
  * Log: `tracing::warn!("Retry #{}: {}", attempt, error)`
* On permanent errors:
  * Log full context: `tracing::error!("Operation failed: {}, User: {}, State: {}", error, user_id, state)`
  * Send admin notification if critical
* User experience:
  * Receives friendly message: «⚠️ Something went wrong. Please try again.»
  * Stays in current FSM state (no reset)
  * Can retry operation
* Session persistence:
  * User data not lost on error
  * FSM state restored after restart
* Admin notifications:
  * Critical errors (DB down, API unreachable) sent to `ADMIN_IDS`
  * Message: «🚨 CRITICAL ERROR: {error_type} — Check logs immediately»
* Logging:
  * All errors logged with `tracing` levels (info/warn/error)
  * Context includes: `user_id`, `chat_id`, `fsm_state`, `operation`
  * SaaS: context also includes `client_id` and tenant-safe identifiers only

## Prompt for AI Agent
Based on `docs/ARCHITECTURE.md` and `docs/ERROR_HANDLING.md`, implement task 0012.

1. Create `src/error.rs`:
   ```rust
   // File: src/error.rs
   use thiserror::Error;

   #[derive(Error, Debug)]
   pub enum AppError {
       #[error("Database error: {0}")]
       Database(#[from] rusqlite::Error),

       #[error("Telegram API error: {0}")]
       Telegram(#[from] teloxide::RequestError),

       #[error("Network error: {0}")]
       Network(#[from] reqwest::Error),

       #[error("Invalid state transition: {0} -> {1}")]
       InvalidStateTransition(String, String),

       #[error("User session expired")]
       SessionExpired,

       #[error("Internal error: {0}")]
       Internal(String),
   }

   impl From<AppError> for teloxide::Payload {
       fn from(err: AppError) -> Self {
           Self::UnknownError(err.to_string())
       }
   }
   ```
2. Create `src/utils/retry.rs`:
   ```rust
   // File: src/utils/retry.rs
   use tokio::time::{sleep, Duration};

   pub async fn with_retry<F, Fut, T, E>(
       operation: F,
       max_attempts: u32,
   ) -> Result<T, E>
   where
       F: Fn() -> Fut,
       Fut: std::future::Future<Output = Result<T, E>>,
       E: std::fmt::Debug,
   {
       for attempt in 1..=max_attempts {
           let result = operation().await;
           if result.is_ok() || attempt == max_attempts {
               return result;
           }
           // Exponential backoff: 1s, 2s, 4s
           let delay = Duration::from_secs(2u64.pow(attempt - 1));
           sleep(delay).await;
       }
       result
   }
   ```
3. In `src/handlers/user.rs`, wrap all DB/API calls with `with_retry`:
   ```rust
   let result = with_retry(
       || async { database::save_appointment(&appointment).await },
       3
   ).await;
   ```
4. In `src/main.rs`, add global error handler:
   * Catch `AppError` in all handlers
   * On error:
     * Send user message: «⚠️ {friendly_message}»
     * Log with full context
     * Notify admin if `AppError::Database` or `AppError::Telegram`
5. Update all handlers to:
   * Use `?` operator with `AppError`
   * Include user context in error messages
   * Preserve FSM state on error
6. Test error scenarios:
   * Simulate DB timeout — verify retry logic
   * Disconnect network — verify fallback message
   * Force Telegram API error — verify admin notification

