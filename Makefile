# Variables for project directories and their corresponding output files
CARGO = cargo
PROJECT_DIRS = cdr_generator
OUTPUT_FILES = cdrs.json

# Default target
all: build

# Build all projects
build: $(PROJECT_DIRS)
	@echo "Building all projects..."
	@for dir in $(PROJECT_DIRS); do \
		cd $$dir && $(CARGO) build --release; \
	done

# Build specific project
build-%:
	@echo "Building project: $*"
	cd /path/to/$* && $(CARGO) build --release

# Run all projects
run: $(PROJECT_DIRS)
	@echo "Running all projects..."
	@for dir in $(PROJECT_DIRS); do \
		cd $$dir && $(CARGO) run --release; \
	done

# Run specific project
run-%:
	@echo "Running project: $*"
	cd /path/to/$* && $(CARGO) run --release

# Clean all projects
clean: $(PROJECT_DIRS)
	@echo "Cleaning all projects..."
	@for dir in $(PROJECT_DIRS); do \
		cd $$dir && $(CARGO) clean; \
		rm -f $$dir/$(OUTPUT_FILES); \
	done

# Clean specific project
clean-%:
	@echo "Cleaning project: $*"
	cd /path/to/$* && $(CARGO) clean
	rm -f /path/to/$*/$(OUTPUT_FILES)

# Format all projects
format: $(PROJECT_DIRS)
	@echo "Formatting all projects..."
	@for dir in $(PROJECT_DIRS); do \
		cd $$dir && $(CARGO) fmt; \
	done

# Lint all projects
lint: $(PROJECT_DIRS)
	@echo "Linting all projects..."
	@for dir in $(PROJECT_DIRS); do \
		cd $$dir && $(CARGO) clippy -- -D warnings; \
	done

# Run tests in all projects
test: $(PROJECT_DIRS)
	@echo "Running tests in all projects..."
	@for dir in $(PROJECT_DIRS); do \
		cd $$dir && $(CARGO) test; \
	done

# Rebuild all projects
rebuild: clean build

# Help
help:
	@echo "Available targets:"
	@echo "  build      - Build all projects"
	@echo "  build-<project_name> - Build a specific project"
	@echo "  run        - Run all projects"
	@echo "  run-<project_name> - Run a specific project"
	@echo "  clean      - Clean all projects"
	@echo "  clean-<project_name> - Clean a specific project"
	@echo "  format     - Format all projects"
	@echo "  lint       - Lint all projects"
	@echo "  test       - Run tests in all projects"
	@echo "  rebuild    - Clean and rebuild all projects"
	@echo "  help       - Show this help message"
