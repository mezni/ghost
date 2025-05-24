# syntax=docker/dockerfile:1

# === Build Stage ===
FROM rustlang/rust:nightly-bookworm as builder

ARG SERVICE_NAME=auth-service

WORKDIR /app

COPY . .

# Build only the specified service
RUN cargo +nightly build --release -p ${SERVICE_NAME}

# === Runtime Stage ===
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

ARG SERVICE_NAME=auth-service

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/${SERVICE_NAME} /app/${SERVICE_NAME}

EXPOSE 3000

ENTRYPOINT ["/app/${SERVICE_NAME}"]
