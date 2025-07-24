# Makefile for Rust Video Editor
# Provides convenient commands for testing, benchmarking, and development

.PHONY: help test test-unit test-integration test-all bench clean fixtures docs lint format check

# Default target
help:
	@echo "Rust Video Editor - Development Commands"
	@echo ""
	@echo "Testing:"
	@echo "  test           - Run all tests"
	@echo "  test-unit      - Run unit tests only"
	@echo "  test-integration - Run integration tests only"
	@echo "  test-all       - Run all tests including ignored FFmpeg tests"
	@echo "  test-coverage  - Generate code coverage report"
	@echo ""
	@echo "Benchmarking:"
	@echo "  bench          - Run all benchmarks"
	@echo "  bench-core     - Run core library benchmarks"
	@echo "  bench-effects  - Run effects benchmarks"
	@echo ""
	@echo "Development:"
	@echo "  check          - Run cargo check on all crates"
	@echo "  lint           - Run clippy linter"
	@echo "  format         - Format code with rustfmt"
	@echo "  docs           - Generate documentation"
	@echo "  clean          - Clean build artifacts"
	@echo ""
	@echo "Test Setup:"
	@echo "  fixtures       - Create test video fixtures (requires FFmpeg)"
	@echo "  install-deps   - Install system dependencies (Ubuntu/Debian)"

# Test commands
test: test-unit test-integration

test-unit:
	@echo "Running unit tests..."
	cargo test --workspace --lib --bins

test-integration:
	@echo "Running integration tests..."
	cargo test --workspace --test '*'

test-all:
	@echo "Running all tests including FFmpeg integration tests..."
	cargo test --workspace --features integration-tests -- --include-ignored

test-coverage:
	@echo "Generating code coverage report..."
	@command -v cargo-llvm-cov >/dev/null 2>&1 || { echo "Installing cargo-llvm-cov..."; cargo install cargo-llvm-cov; }
	cargo llvm-cov --workspace --html
	@echo "Coverage report generated in target/llvm-cov/html/"

# Benchmark commands
bench:
	@echo "Running all benchmarks..."
	$(MAKE) bench-core

bench-core:
	@echo "Running core library benchmarks..."
	cd crates/core && cargo bench --bench frame_benchmarks
	cd crates/core && cargo bench --bench buffer_benchmarks

bench-effects:
	@echo "Running effects benchmarks..."
	cd crates/effects && cargo bench --bench effects_benchmark

# Development commands
check:
	@echo "Running cargo check..."
	cargo check --workspace --all-targets --all-features

lint:
	@echo "Running clippy linter..."
	cargo clippy --workspace --all-targets --all-features -- -D warnings

format:
	@echo "Formatting code..."
	cargo fmt --all

format-check:
	@echo "Checking code formatting..."
	cargo fmt --all -- --check

docs:
	@echo "Generating documentation..."
	cargo doc --workspace --no-deps --document-private-items --open

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Test setup commands
fixtures:
	@echo "Creating test video fixtures..."
	@command -v ffmpeg >/dev/null 2>&1 || { echo "FFmpeg is required to create fixtures. Please install FFmpeg first."; exit 1; }
	cd tests/fixtures && python3 create_test_videos.py

install-deps:
	@echo "Installing system dependencies (Ubuntu/Debian)..."
	sudo apt-get update
	sudo apt-get install -y \
		ffmpeg \
		libavcodec-dev \
		libavformat-dev \
		libavutil-dev \
		libswscale-dev \
		libavfilter-dev \
		libavdevice-dev \
		pkg-config \
		python3

# Rust installation helpers
install-rust-tools:
	@echo "Installing additional Rust tools..."
	cargo install cargo-llvm-cov cargo-audit cargo-udeps

# Quick development cycle
dev-check: format-check lint test-unit
	@echo "✅ Development checks passed!"

# Full CI simulation
ci-local: format-check lint test-all
	@echo "✅ All CI checks passed!"

# Performance testing
perf-test:
	@echo "Running performance tests..."
	cargo test --workspace --release performance_tests

# Memory usage testing
memory-test:
	@echo "Running memory usage tests..."
	@command -v valgrind >/dev/null 2>&1 || { echo "Valgrind not found. Install with: sudo apt-get install valgrind"; exit 1; }
	cargo build --workspace --tests
	valgrind --tool=massif --detailed-freq=1 \
		cargo test --workspace --release test_memory_pool_under_load

# Docker testing environment
docker-test:
	@echo "Running tests in Docker container..."
	docker build -t rust-video-editor-test -f docker/Dockerfile.test .
	docker run --rm -v $(PWD):/workspace rust-video-editor-test make test-all

# Profile guided optimization build
pgo-build:
	@echo "Building with Profile Guided Optimization..."
	cargo build --release
	# Run benchmarks to generate profile data
	$(MAKE) bench
	# Rebuild with PGO data
	RUSTFLAGS="-Cprofile-use=target/pgo-data" cargo build --release

# Security audit
security-audit:
	@echo "Running security audit..."
	@command -v cargo-audit >/dev/null 2>&1 || { echo "Installing cargo-audit..."; cargo install cargo-audit; }
	cargo audit

# Check for outdated dependencies
outdated:
	@echo "Checking for outdated dependencies..."
	@command -v cargo-outdated >/dev/null 2>&1 || { echo "Installing cargo-outdated..."; cargo install cargo-outdated; }
	cargo outdated --workspace

# Update dependencies
update-deps:
	@echo "Updating dependencies..."
	cargo update

# Build for different targets
build-targets:
	@echo "Building for multiple targets..."
	cargo build --target x86_64-unknown-linux-gnu
	cargo build --target x86_64-pc-windows-gnu
	cargo build --target x86_64-apple-darwin

# Generate dependency graph
deps-graph:
	@echo "Generating dependency graph..."
	@command -v cargo-deps >/dev/null 2>&1 || { echo "Installing cargo-deps..."; cargo install cargo-deps; }
	cargo deps --all-deps | dot -Tpng > dependencies.png
	@echo "Dependency graph saved as dependencies.png"

# Bundle for release
bundle:
	@echo "Creating release bundle..."
	cargo build --release --workspace
	mkdir -p release/
	cp target/release/rust-video-editor release/ 2>/dev/null || true
	cp README.md LICENSE release/
	tar -czf rust-video-editor-release.tar.gz release/
	@echo "Release bundle created: rust-video-editor-release.tar.gz"