FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Force both cargo builds to share the same target directory
ENV CARGO_TARGET_DIR=/app/target

# Copy all manifests first for dependency caching
COPY backend/Cargo.toml backend/Cargo.lock backend/
COPY discord-bot/Cargo.toml discord-bot/Cargo.lock discord-bot/

# Create dummy main files to cache dependencies
RUN mkdir -p backend/src discord-bot/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "fn main() {}" > discord-bot/src/main.rs

# Build dependencies (cached layer)
RUN cargo build --release --manifest-path backend/Cargo.toml 2>/dev/null; return 0
RUN cargo build --release --manifest-path discord-bot/Cargo.toml 2>/dev/null; return 0

# Now copy the real source
COPY backend/src backend/src/
COPY discord-bot/src discord-bot/src/

# Touch the main files to force recompilation
RUN touch backend/src/main.rs discord-bot/src/main.rs

# Build both binaries
RUN cargo build --release --manifest-path backend/Cargo.toml && \
    cargo build --release --manifest-path discord-bot/Cargo.toml

# ---- Runtime stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binaries
COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/target/release/discord-bot /app/discord-bot

# Create beatmap cache directory
RUN mkdir -p /app/maps

# Copy entrypoint script
COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

EXPOSE 8000

ENTRYPOINT ["/app/docker-entrypoint.sh"]
