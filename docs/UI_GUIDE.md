# UI Design Guide

This guide applies to both distribution modes:

- **SaaS (multi-tenant)**: UI copy may be tenant-branded; actions must never reference or expose other tenants.
- **On‑Prem (single-tenant)**: UI is typically fully branded for one customer.

## Principles

* Keep messages short and clear.
* Use inline keyboards for navigation.
* Highlight important info (bold/italic).

## Keyboard Rules

* Max 3 buttons per row.
* Use `CallbackQuery` for selections.
* Update message instead of sending new one.

## Message Formatting

* Use HTML for bold/italic: `<b>Bold</b>`, `<i>Italic</i>`.
* Limit message length to 4096 chars.

## Hybrid UX rules (SaaS vs On‑Prem)

### Tenant-safe messaging (SaaS)

- Never display internal identifiers (`client_id`, database IDs).
- Avoid copy that implies “global” admin access across tenants.
- If support/admin features exist, ensure wording is “your workspace/salon” (tenant scoped).

### Branding

- Prefer tenant-provided display name (e.g., salon name) when available.
- Keep a sane default brand (“NailBot”) if tenant branding is missing.

## Examples

**Main Menu:**
```
🤖 NailBot — Your Nail Appointment Assistant

Choose an option:
• 🗓️ Book an appointment
• 👤 My appointments
• 💰 Price list
• 📸 Portfolio
```

**Time Slot Selection:**
```
Available slots for 2024-05-20:

• 10:00 — ✅ Free
• 11:30 — ❌ Busy
• 13:00 — ✅ Free
```