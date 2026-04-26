# Task 0000: Finalize PostgreSQL Choice and RU/EN i18n Strategy

## Goal
Convert project documentation to an explicit **PostgreSQL-only** backend direction and define a production-ready **multi-language strategy (RU/EN)** compatible with the API-first architecture.

## Context
This task is documentation-only and should be completed before implementation-heavy tasks. It establishes shared standards so future development does not diverge on DB technology or localization approach.

## API/Bot Split Alignment
* [ ] PostgreSQL access and migrations are backend/API concerns.
* [ ] Telegram bot acts as a localization-aware client adapter over API responses/use-cases.
* [ ] i18n key catalogs are shared by all channels (Telegram now, web/mobile later).

## Requirements

### 1) PostgreSQL Standardization
* [ ] Update documentation to treat PostgreSQL as the canonical database backend.
* [ ] Remove/replace guidance that positions SQLite as primary runtime DB.
* [ ] Align architecture and setup docs with:
  * [ ] Postgres connection pooling
  * [ ] migrations lifecycle
  * [ ] tenant-safe query patterns via `client_id`
* [ ] Ensure compose/deploy docs describe API + bot + Postgres topology.

### 2) i18n Library and Message Strategy (RU/EN)
* [ ] Select Rust i18n approach and document rationale:
  * [ ] `fluent` + `fluent-bundle` for translation catalogs and runtime locale switching
  * [ ] `unic-langid` for locale identifiers and fallback handling
* [ ] Define locale resolution policy:
  * [ ] SaaS: per-user locale stored in DB and loaded per request/session
  * [ ] On-Prem: default locale from `.env` with optional per-user override
* [ ] Move user-facing text to external localization resources (`locales/ru.ftl`, `locales/en.ftl`).
* [ ] Keep business logic language-neutral; only adapters/renderers format localized strings.

### 3) Documentation Sync
* [ ] Update docs and task checklists to mention:
  * [ ] PostgreSQL migration flow
  * [ ] i18n key usage instead of hardcoded strings
  * [ ] API + bot separation implications for localization
* [ ] Update `AGENTS.md` router hints so future tasks follow PostgreSQL + i18n baseline.

### 4) Hybrid Model Clarity
* [ ] SaaS documentation clearly states:
  * [ ] multi-tenant behavior (`client_id` everywhere)
  * [ ] locale can vary by user
* [ ] On-Prem documentation clearly states:
  * [ ] single-tenant behavior (`client_id = 1`)
  * [ ] default locale policy from environment
  * [ ] license checks remain required

## Acceptance Criteria
* PostgreSQL is documented as the only default backend for production architecture.
* Core docs are consistent on API-first split and bot-as-adapter model.
* RU/EN i18n approach, library choice, and locale resolution rules are documented.
* No conflicting guidance remains about where translations live or how locale is selected.
* Task docs reference localization and migration concerns where relevant.

## Output Format for This Task
When reporting completion, provide:

1. Short summary of modified documentation files.
2. A brief Rust example of i18n integration in a basic handler.

Example snippet (documentation sample):

```rust
use fluent_bundle::{FluentBundle, FluentResource};
use unic_langid::langid;

fn tr_hello(locale: &str) -> String {
    let lang = locale.parse().unwrap_or(langid!("en"));
    let mut bundle = FluentBundle::new_concurrent(vec![lang]);
    let res = FluentResource::try_new("hello = Hello, { $name }!".to_string()).unwrap();
    bundle.add_resource(res).unwrap();

    let msg = bundle.get_message("hello").unwrap();
    let pattern = msg.value().unwrap();
    let mut errors = vec![];
    bundle.format_pattern(pattern, None, &mut errors).to_string()
}
```

