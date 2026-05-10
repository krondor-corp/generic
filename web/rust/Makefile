ARGS ?=

PROJECT_ROOT := $(shell cd .. && pwd)
DB_DIR := data

.PHONY: help
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@echo '  dev:        Run the dev server with hot reload (vault secrets)'
	@echo '  check:      Check all Rust code'
	@echo '  install:    Install dependencies'
	@echo '  build:      Build all Rust packages'
	@echo '  test:       Run all tests'
	@echo '  lint:       Run clippy linter'
	@echo '  fmt:        Format code'
	@echo '  fmt-check:  Check code formatting'
	@echo '  db-clean:   Delete the SQLite database'
	@echo '  db-reset:   Reset the database'
	@echo '  clean:      Clean build artifacts'

.PHONY: dev
DEV_PORT ?= 9000

dev: ## Run the dev server with hot reload
	@mkdir -p $(DB_DIR)
	@confit log "Starting on http://localhost:$(DEV_PORT) ..."
	@env SQLITE_DATABASE_URL=sqlite://data/app.db \
		LISTEN_ADDRESS=127.0.0.1:$(DEV_PORT) \
		HOST_NAME=http://localhost:$(DEV_PORT) \
		SERVICE_SECRET=dev-secret-change-in-prod \
		confit --set stage=development run --upper credentials.app -- \
		cargo watch -x "run --bin generic-rust-app"

# -- Database targets (SQLite) --

.PHONY: db-clean
db-clean: ## Delete the database
	@rm -f $(DB_DIR)/app.db
	@echo "Database cleaned"

.PHONY: db-reset
db-reset: db-clean ## Reset the database
	@mkdir -p $(DB_DIR)
	@echo "Database reset"

# -- Standard targets --

.PHONY: check
check: ## Check all Rust code
	cargo check --all

.PHONY: install
install: ## Install dependencies
	cargo fetch

.PHONY: build
build: ## Build all Rust packages
	cargo build --all

.PHONY: test
test: ## Run all tests
	cargo test --all

.PHONY: lint
lint: ## Run clippy linter
	cargo clippy --all -- -D warnings

.PHONY: fmt
fmt: ## Format code
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Check code formatting
	cargo fmt --all -- --check

.PHONY: types
types: ## Run type checking (alias for check)
	@$(MAKE) check

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean
