# Variables
CARGO = cargo
APP_NAME = ghost  
BUILD_DIR = target

# Default target
all: build

# Build the project
build:
	$(CARGO) build

# Build in release mode
release:
	$(CARGO) build --release

# Run the application
run:
	$(CARGO) run

# Run in release mode
run-release:
	$(CARGO) run --release

# Run tests
test:
	$(CARGO) test

# Check for linting issues
lint:
	$(CARGO) clippy -- -D warnings

# Format code
format:
	$(CARGO) fmt

# Clean build artifacts
clean:
	$(CARGO) clean

# Install dependencies
install-deps:
	rustup update
	$(CARGO) install clippy
	$(CARGO) install rustfmt

# Display help
help:
	@echo "Makefile for Rust project:"
	@echo "  make          - Build the project (default)"
	@echo "  make build    - Build the project"
	@echo "  make release  - Build in release mode"
	@echo "  make run      - Run the application"
	@echo "  make run-release - Run the application in release mode"
	@echo "  make test     - Run tests"
	@echo "  make lint     - Check for linting issues using Clippy"
	@echo "  make format   - Format the code using rustfmt"
	@echo "  make clean    - Clean the build artifacts"
	@echo "  make install-deps - Install/update dependencies (Rust, Clippy, rustfmt)"

# PHONY targets (not associated with files)
.PHONY: all build release run run-release test lint format clean install-deps help
