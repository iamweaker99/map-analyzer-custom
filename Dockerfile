# ── Stage 1: Build Rust binaries ──
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /app

# Backend
COPY backend/Cargo.toml backend/Cargo.lock ./backend/
COPY backend/src ./backend/src

# Discord bot
COPY discord-bot/Cargo.toml discord-bot/Cargo.lock ./discord-bot/
COPY discord-bot/src ./discord-bot/src

# Build both binaries
RUN cargo build --release --manifest-path backend/Cargo.toml && \
    cargo build --release --manifest-path discord-bot/Cargo.toml

# ── Stage 2: Runtime ──
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Create beatmap cache directory
RUN mkdir -p /app/maps

# Copy compiled binaries
COPY --from=builder /app/backend/target/release/backend .
COPY --from=builder /app/discord-bot/target/release/discord-bot .

# Copy entrypoint
COPY docker-entrypoint.sh .
RUN chmod +x docker-entrypoint.sh

EXPOSE 8000

ENTRYPOINT ["/app/docker-entrypoint.sh"]
