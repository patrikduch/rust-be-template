# Build Stage
FROM rust:slim as builder

WORKDIR /usr/src/app

# Install musl tools for static linking
RUN apt-get update && apt-get install -y --no-install-recommends \
    musl-tools \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set target to musl for static binary
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN rustup target add x86_64-unknown-linux-musl

# Copy just the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy project structure for dependency caching
RUN mkdir -p src
# Create a dummy main.rs with the same dependencies as your real app
RUN echo 'fn main() { println!("Dummy"); }' > src/main.rs

# Build dependencies only
RUN cargo build --release --target x86_64-unknown-linux-musl

# Remove the build artifacts that depend on the dummy file
RUN rm -rf target/x86_64-unknown-linux-musl/release/deps/rust_be_template*
RUN rm -rf target/x86_64-unknown-linux-musl/release/rust-be-template*
RUN rm -f src/main.rs

# Now copy the actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime Stage
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies
RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

# Create non-root user
RUN addgroup -S appuser && adduser -S appuser -G appuser

# Copy the binary from builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rust-be-template /app/rust-be-template

# Set proper permissions
RUN chown -R appuser:appuser /app && chmod +x /app/rust-be-template

# Switch to non-root user
USER appuser

# Expose the port your application uses (Actix typically uses 8080)
# Expose port 80 for Azure App Service
EXPOSE 80

# Set environment variables
ENV TZ=Etc/UTC
ENV RUST_LOG=info
ENV PORT=80

# Run the application
CMD ["/app/rust-be-template"]