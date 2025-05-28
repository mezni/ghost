# Use the official Rust image with the specified edition
FROM rust:alpine AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory to avoid copying src files yet
RUN mkdir src

# Copy the source code
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Use a smaller base image for the final image
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/auth-service .

# Expose the port
EXPOSE 3200

# Run the command to start the service
CMD ["./auth-service"]