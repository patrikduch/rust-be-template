# Stage 1: Build
FROM rust:slim AS builder

WORKDIR /usr/src/app

# Install musl & build deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    musl-tools \
    pkg-config \
    curl \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Add musl target
RUN rustup target add x86_64-unknown-linux-musl

# Env for static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"

# Pre-copy/build deps for better caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    touch src/lib.rs && \
    echo "fn main() {println!(\"dummy\");}" > src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/rust-be-template

# Stage 2: Runtime
FROM alpine:latest

# Add CA certificates and timezone data
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

WORKDIR /app

# Copy only the executable from builder
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rust-be-template /app/

# Use non-root user
USER appuser

# Set executable as entrypoint
ENTRYPOINT ["/app/rust-be-template"]