# Stage 1: Build
FROM rust:slim AS builder

WORKDIR /usr/src/app

# Define build argument
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Install musl & build deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    musl-tools \
    pkg-config \
    curl \
    libssl-dev \
    uuid-dev \
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
RUN apk add --no-cache ca-certificates tzdata libcap

# Copy build arg to runtime stage
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

WORKDIR /app

# Copy only the executable from builder
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rust-be-template /app/

# Make sure the executable is executable and has port binding capability
RUN chmod +x /app/rust-be-template && \
    setcap 'cap_net_bind_service=+ep' /app/rust-be-template

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup && \
    chown -R appuser:appgroup /app

# Expose port 80 for Azure WebApp
EXPOSE 80

# Use non-root user
USER appuser

# Set executable as entrypoint
ENTRYPOINT ["/app/rust-be-template"]