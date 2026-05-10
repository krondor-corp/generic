.PHONY: help
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*##"; printf "\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-20s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: install
install: ## Install dependencies
	@pnpm install

.PHONY: dev
dev: ## Run development servers (all apps)
	@pnpm dev

.PHONY: build
build: ## Build all projects
	@pnpm build

.PHONY: test
test: ## Run all tests
	@pnpm test

.PHONY: fmt
fmt: ## Format code
	@pnpm fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting
	@pnpm fmt:check

.PHONY: types
types: ## Check TypeScript types
	@pnpm check-types

.PHONY: lint
lint: ## Run linter
	@echo "no lint for ts"

.PHONY: check
check: fmt-check types test ## Run all checks

.PHONY: clean
clean: ## Clean build artifacts, dependencies, and node_modules
	@echo "Cleaning build artifacts and turbo cache..."
	@pnpm clean
	@echo "Cleaning built asset symlinks..."
	@rm -f apps/web/public/favicon.ico apps/web/public/favicon.png apps/web/public/icon.png apps/web/public/icon.svg
	@echo "Cleaning node_modules..."
	@rm -rf node_modules apps/*/node_modules packages/*/node_modules
	@echo "Cleaning turbo cache..."
	@rm -rf .turbo
	@echo "✓ Clean complete"

.PHONY: docker-build
docker-build: ## Build Docker images
	@$(MAKE) -j2 docker-build-api docker-build-web

docker-build-%: ## Build Docker image for the given service
	@./bin/docker build $(@:docker-build-%=%)