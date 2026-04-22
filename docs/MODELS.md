# Data Models Guide

## Rust Structs

### `User`

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub telegram_id: i64,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

### `Appointment`

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Appointment {
    pub id: Option<i64>,
    pub user_id: i64,
    pub date: String, // YYYY-MM-DD
    pub time: String, // HH:MM
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

### `TimeSlot`

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimeSlot {
    pub id: Option<i64>,
    pub date: String, // YYYY-MM-DD
    pub time: String, // HH:MM
    pub is_available: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

## Rules

* Use `Option<T>` for nullable fields.
* Use `chrono::DateTime<Utc>` for timestamps.
* Derive `serde` traits for serialization.
* Keep field names snake_case.

## AI Prompts

> Define `User` struct in `src/database/models.rs`. Include fields: id, telegram_id, name, phone, created_at. Derive Debug, Clone, Serialize, Deserialize. Add documentation comments.
