# syntax=docker/dockerfile:1

FROM rust:1.78-bookworm AS builder

WORKDIR /app

# Cache deps first
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/nailbot /app/nailbot

# Default data directory for SQLite and other runtime files
RUN mkdir -p /app/data

ENV RUST_LOG=info

VOLUME ["/app/data"]

CMD ["/app/nailbot"]
