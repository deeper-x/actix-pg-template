PHONY: run

# Set environment variables
RUST_BACKTRACE := 1
RUST_LOG := actix_web=debug

# Default target: run the project
run:
	RUST_BACKTRACE=${RUST_BACKTRACE} RUST_LOG=${RUST_LOG} cargo run

# Build the project
build:
	cargo build

# Build the project in release mode
release:
	cargo build --release

# Test the project
test:
	cargo test

# Clean the project (remove target directory)
clean:
	cargo clean

# Format the code using rustfmt
fmt:
	cargo fmt

# Check code formatting
fmt-check:
	cargo fmt -- --check

# Run Clippy for linting
lint:
	cargo clippy

# Run Clippy with all lints enabled
lint-all:
	cargo clippy -- -W clippy::all

# Run the project with backtraces enabled
run-backtrace:
	RUST_BACKTRACE=1 cargo run
