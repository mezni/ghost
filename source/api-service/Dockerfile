# === 1. Build Stage ===
FROM rust:1.76 as builder

# Create app directory
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -r src

# Copy actual source code and build again
COPY . .
RUN cargo build --release

# === 2. Runtime Stage ===
FROM debian:bullseye-slim

# Install required system dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy built binary from build stage
COPY --from=builder /app/target/release/api-service /usr/local/bin/api-service

# Expose the port your Rust server listens on
EXPOSE 3000

# Start the application
CMD ["api-service"]
