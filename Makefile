# Trellis Makefile
#
# Primary entry point for building and testing the Trellis integrity substrate.

# Basic configuration
PYTHON = python3
CARGO = cargo
PYTEST = $(PYTHON) -m pytest

# Paths
TRELLIS_PY_DIR = trellis-py
SCRIPTS_DIR = scripts
VECTORS_DIR = fixtures/vectors

.PHONY: all help build test test-rust test-python test-scripts test-postgres check-specs check-specs-strict lint fmt clean

all: build test

help:
	@echo "Trellis Build & Test Tool"
	@echo ""
	@echo "Usage:"
	@echo "  make build          Build all Rust crates"
	@echo "  make test           Run all tests (Rust, Python, Scripts, Specs)"
	@echo "  make test-rust      Run all Rust tests"
	@echo "  make test-python    Run Python conformance tests"
	@echo "  make test-scripts   Run tests for helper scripts"
	@echo "  make test-postgres  Run trellis-store-postgres + parity integration tests (needs initdb/pg_ctl on PATH)"
	@echo "  make check-specs    Run spec discipline and coverage lint"
	@echo "  make check-specs-strict  Run check-specs + vector-renumbering guard (CI variant)"
	@echo "  make lint           Run Rust clippy"
	@echo "  make fmt            Check Rust formatting"
	@echo "  make clean          Clean build artifacts"
	@echo ""

build:
	@echo "Building Rust workspace..."
	$(CARGO) build --workspace

test: test-rust test-python test-scripts check-specs

test-rust:
	@echo "Running Rust tests..."
	$(CARGO) test --workspace

test-python:
	@echo "Running Python conformance tests..."
	cd $(TRELLIS_PY_DIR) && PYTHONPATH=src $(PYTHON) -m trellis_py.conformance --vectors ../$(VECTORS_DIR)
	@echo "Checking for Python unit tests..."
	-cd $(TRELLIS_PY_DIR) && $(PYTEST) -q

test-scripts:
	@echo "Running script tests..."
	$(PYTHON) $(SCRIPTS_DIR)/test_check_specs.py
	$(PYTHON) $(SCRIPTS_DIR)/test_check_vector_renumbering.py

# Targeted run of the Postgres-side integration suite. `cargo test --workspace`
# already exercises these — this target exists for fast iteration on the
# canonical-side of the wos-server EventStore composition (per VISION.md §V).
# Requires `initdb` and `pg_ctl` discoverable on PATH (Postgres 14+; tested
# against Postgres 16 via `/opt/homebrew/opt/postgresql@16/bin`).
test-postgres:
	@echo "Running trellis-store-postgres + parity integration tests..."
	$(CARGO) test -p trellis-store-postgres
	$(CARGO) test -p trellis-conformance --test store_parity

check-specs:
	@echo "Running spec checks..."
	$(PYTHON) $(SCRIPTS_DIR)/check-specs.py

# Strict pre-merge variant: enables the vector-renumbering guard
# (`TRELLIS_CHECK_RENUMBERING=1`) that compares vector NNN- prefixes against
# the base ref (default `origin/main`, override via `TRELLIS_RATIFICATION_REF`).
# CI invokes this target; local dev uses `check-specs` (without the guard).
check-specs-strict:
	@echo "Running spec checks (strict, with renumbering guard)..."
	TRELLIS_CHECK_RENUMBERING=1 $(PYTHON) $(SCRIPTS_DIR)/check-specs.py

lint:
	@echo "Running Rust clippy..."
	$(CARGO) clippy --workspace -- -D warnings

fmt:
	@echo "Checking Rust formatting..."
	$(CARGO) fmt --all -- --check

clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	find . -name "__pycache__" -type d -exec rm -rf {} +
	rm -f $(TRELLIS_PY_DIR)/BYTE-MATCH-REPORT.json
