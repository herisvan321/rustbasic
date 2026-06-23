# ============================================================
# RustBasic Docker Build — Standalone
# ============================================================

# Stage 1: Builder
FROM rust:1-slim-bookworm AS builder

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    apt-get update && apt-get install -y \
    pkg-config libssl-dev

# Copy rustbasic-core dependency
WORKDIR /rustbasic-core
COPY --from=core . .

WORKDIR /build

# Copy source code
COPY . .

# Build release binary using Cargo registry, git cache, and target cache
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    cargo build --release --bin rustbasic && \
    cp target/release/rustbasic /build/rustbasic-bin

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary dari builder stage
COPY --from=builder /build/rustbasic-bin ./rustbasic

# Copy assets yang diperlukan dari builder stage
COPY --from=builder /build/src/resources/views/ src/resources/views/
COPY --from=builder /build/src/dist/ src/dist/
COPY --from=builder /build/public/ public/
COPY --from=builder /build/database/ database/
COPY --from=builder /build/.env.example .env

# Expose port aplikasi
EXPOSE 4000

CMD ["./rustbasic"]
