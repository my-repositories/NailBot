# UI Design Guide

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