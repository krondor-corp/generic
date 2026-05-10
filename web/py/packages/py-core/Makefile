.PHONY: help
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*##"; printf "\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-20s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

WEB_ROOT := $(shell cd ../../.. && pwd)
PROJECT_ROOT := $(shell cd ../../../.. && pwd)
POSTGRES_URL := $(shell confit resolve accessories.postgres.url 2>/dev/null)

ifdef POSTGRES_URL
  POSTGRES_ENV := POSTGRES_URL="$(POSTGRES_URL)"
else
  POSTGRES_ENV :=
endif

.PHONY: setup
setup: ## Start postgres + redis containers, migrate, and seed
	@$(WEB_ROOT)/bin/postgres up
	@$(WEB_ROOT)/bin/redis up
	@$(WEB_ROOT)/bin/postgres ensure_database
	@$(POSTGRES_ENV) ./bin/db migrate
	@$(POSTGRES_ENV) ./bin/seed all

.PHONY: seed
seed: ## Seed database with initial data
	@$(POSTGRES_ENV) ./bin/seed all

.PHONY: seed-users
seed-users: ## Seed users only
	@./bin/seed users

.PHONY: seed-dry-run
seed-dry-run: ## Show what would be seeded (dry run)
	@./bin/seed --dry-run

.PHONY: teardown
teardown: ## Stop and remove all containers
	@$(WEB_ROOT)/bin/postgres down
	@$(WEB_ROOT)/bin/redis down

.PHONY: wipe
wipe: ## Drop the branch-specific database (keeps container running)
	@$(WEB_ROOT)/bin/postgres wipe

.PHONY: test
test: ## Run tests
	@$(POSTGRES_ENV) uv run pytest

.PHONY: lint
lint: ## Run linting
	@uv run ruff check src/ tests/

.PHONY: fmt
fmt: ## Format code
	@uv run black src/ tests/

.PHONY: fmt-check
fmt-check: ## Check formatting
	@uv run black --check src/ tests/

.PHONY: types
types: ## Run type checking with ty
	@uv run ty check src/ tests/

.PHONY: check
check: fmt-check lint types test ## Run all checks

.PHONY: clean
clean: ## Clean build artifacts
	@find . -type d -name "__pycache__" -exec rm -rf {} +
	@find . -type d -name ".pytest_cache" -exec rm -rf {} +
	@find . -type d -name ".ruff_cache" -exec rm -rf {} +
	@find . -type f -name "*.pyc" -delete

# Database commands
.PHONY: db-up
db-up: ## Start PostgreSQL container
	@$(WEB_ROOT)/bin/postgres up

.PHONY: db-down
db-down: ## Stop and remove PostgreSQL container (deletes data)
	@$(WEB_ROOT)/bin/postgres down

.PHONY: db-status
db-status: ## Check PostgreSQL container status
	@$(WEB_ROOT)/bin/postgres status

.PHONY: db-endpoint
db-endpoint: ## Print PostgreSQL connection URL
	@$(WEB_ROOT)/bin/postgres endpoint

.PHONY: db-connect
db-connect: ## Connect to PostgreSQL with psql
	@$(WEB_ROOT)/bin/postgres connect

.PHONY: db-migrate
db-migrate: ## Run pending database migrations
	@$(POSTGRES_ENV) ./bin/db migrate

.PHONY: db-prepare
db-prepare: ## Create new migration (usage: make db-prepare MSG="description")
ifdef MANUAL
	@$(POSTGRES_ENV) ./bin/db prepare --manual "$(MSG)"
else
	@$(POSTGRES_ENV) ./bin/db prepare "$(MSG)"
endif
