# Subscription Check Guide

## Logic

1. User triggers action requiring subscription (e.g., booking).
2. Bot calls `getChatMember` Telegram API method.
3. Check status: `member`, `administrator`, `creator` → subscribed.
4. If not subscribed: show message and link to channel.
5. Log result.

## Required Data

* `CHANNEL_ID` from config (`src/config.rs`).
* `user_id` from message context.

## Functions

* `check_subscription(user_id: i64) -> Result<bool, Box<dyn std::error::Error>>`
* Returns `true` if subscribed, `false` otherwise.

## Error Handling

* Network errors → retry logic (3 attempts).
* Invalid `CHANNEL_ID` → log error, return `false`.
* Telegram API rate limits → exponential backoff.

## AI Prompts

> Implement `check_subscription` function in `src/utils/subscription.rs`. Use `teloxide::Bot::get_chat_member`. Check user status in `CHANNEL_ID`. Return `true` for `Member`, `Administrator`, `Creator`. Log result and errors. Add retry logic on network failure.

> Create `require_subscription` middleware. Wrap booking handlers. If not subscribed, send message: «To book, please subscribe to our channel: <link>». Use `CHANNEL_LINK` from config.


## UI Messages

* **Not subscribed:**
  ```
  ⚠️ To book an appointment, please subscribe to our official channel:

  👉 <CHANNEL_LINK>

  After subscribing, click «✅ I'm subscribed» to continue.
  ```
* **Subscribed:** Proceed with booking flow.
* **Error:** «Sorry, we can't verify your subscription now. Please try again later.»

## Testing

* Test cases:
  * User is member → expect `true`.
  * User left channel → expect `false`.
  * Invalid `CHANNEL_ID` → expect error log, `false`.
  * Network timeout → expect retry, then `false`.
