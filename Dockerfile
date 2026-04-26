# syntax=docker/dockerfile:1

FROM rust:1.78-bookworm AS builder

WORKDIR /app

# Cache deps first
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release --bin nailbot-api --bin nailbot-bot

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

# Default data directory for SQLite and other runtime files
RUN mkdir -p /app/data

ENV RUST_LOG=info

VOLUME ["/app/data"]

FROM runtime AS api
COPY --from=builder /app/target/release/nailbot-api /app/nailbot-api
EXPOSE 8080
CMD ["/app/nailbot-api"]

FROM runtime AS bot
COPY --from=builder /app/target/release/nailbot-bot /app/nailbot-bot
CMD ["/app/nailbot-bot"]
