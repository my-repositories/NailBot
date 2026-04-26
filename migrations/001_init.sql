CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    client_id BIGINT NOT NULL,
    telegram_id BIGINT NOT NULL,
    full_name TEXT,
    phone TEXT,
    locale TEXT NOT NULL DEFAULT 'en',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (client_id, telegram_id)
);

CREATE TABLE IF NOT EXISTS appointments (
    id BIGSERIAL PRIMARY KEY,
    client_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    appointment_date DATE NOT NULL,
    appointment_time TIME NOT NULL,
    service_type TEXT NOT NULL DEFAULT 'Nail service',
    status TEXT NOT NULL DEFAULT 'confirmed',
    reminder_24h_sent BOOLEAN NOT NULL DEFAULT FALSE,
    reminder_1h_sent BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (client_id, appointment_date, appointment_time)
);

CREATE TABLE IF NOT EXISTS time_slots (
    id BIGSERIAL PRIMARY KEY,
    client_id BIGINT NOT NULL,
    slot_date DATE NOT NULL,
    slot_time TIME NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE (client_id, slot_date, slot_time)
);

CREATE INDEX IF NOT EXISTS idx_appointments_client_user
    ON appointments(client_id, user_id);
