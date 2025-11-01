# syntax=docker/dockerfile:1

# Use nightly or stable depending on edition (2024 requires nightly)
FROM rustlang/rust:nightly-bookworm as builder

WORKDIR /app

# Copy the back-service directory
COPY source/back-service/ .  

# Build the binary
RUN cargo +nightly build --release

# Runtime stage
FROM debian:bookworm-slim

# PostgreSQL client libs if needed at runtime
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/back-service /app/back-service

ENTRYPOINT ["/app/back-service"]