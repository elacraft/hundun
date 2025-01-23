# Makefile for Hundun project

# Variables
CARGO = cargo
PROJECT_NAME = hundun
TARGET = target
RELEASE_TARGET = $(TARGET)/release
DEBUG_TARGET = $(TARGET)/debug
BIN_NAME = $(PROJECT_NAME)

# Default target
all: build

# Build the project in debug mode
build:
	$(CARGO) build

# Build the project in release mode
release:
	$(CARGO) build --release

# Run the project
run:
	$(CARGO) run

# Run tests
test:
	$(CARGO) test --verbose

# Clean the project
clean:
	$(CARGO) clean

# Format the code
fmt:
	$(CARGO) fmt

# Check for errors without building
check:
	$(CARGO) check

# Lint the code
clippy:
	$(CARGO) clippy

# Generate documentation
doc:
	$(CARGO) doc --open

# Install dependencies
update:
	$(CARGO) update

# Run benchmarks
bench:
	$(CARGO) bench

# Run a specific example
# Usage: make example EXAMPLE=example_name
example:
	$(CARGO) run --example $(EXAMPLE)

# Run a specific test
# Usage: make test-single TEST=test_name
test-single:
	$(CARGO) test $(TEST)

test-hurl:
	hurl hundun/tests/*.hurl

# Build and run the project in release mode
run-release: release
	$(RELEASE_TARGET)/$(BIN_NAME)

# Build and run the project in debug mode
run-debug: build
	$(DEBUG_TARGET)/$(BIN_NAME)

# Phony targets
.PHONY: all build release run test clean fmt check clippy doc update bench example test-single test-hurl run-release run-debug