# syntax=docker/dockerfile:1
# --- Stage 1: Базовый образ с cargo-chef ---
FROM rustlang/rust:nightly-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef

# --- Stage 2: Планирование (анализ зависимостей) ---
FROM chef AS planner
COPY . .
# Генерируем рецепт кэша
RUN cargo chef prepare --recipe-path recipe.json

# --- Stage 3: Сборка зависимостей ---
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
ENV CARGO_UNSTABLE_EDITION2024=true
# Собираем только библиотеки (этот слой закешируется)
RUN cargo chef cook --release --recipe-path recipe.json

# --- Stage 4: Сборка самих бинарников ---
COPY . .
ENV CARGO_UNSTABLE_EDITION2024=true
RUN cargo build --release --bin nailbot-api --bin nailbot-bot

# --- Stage 5: Общий Runtime образ ---
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Установка необходимых системных библиотек (как в вашем оригинале)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Создаем папку для данных и настраиваем логи
RUN mkdir -p /app/data
ENV RUST_LOG=info
VOLUME ["/app/data"]

# --- Stage 6: Финальный образ для API ---
FROM runtime AS api
COPY --from=builder /app/target/release/nailbot-api /app/nailbot-api
EXPOSE 8080
CMD ["/app/nailbot-api"]

# --- Stage 7: Финальный образ для BOT ---
FROM runtime AS bot
COPY --from=builder /app/target/release/nailbot-bot /app/nailbot-bot
CMD ["/app/nailbot-bot"]
