ARGS ?=

# Tauri stack: a Rust workspace (std/api, std/rs, server, app backend) and
# a pnpm workspace (std/ts, app frontend). The standard targets fan out to
# both; dev-* pick a target platform for the example app.

APP_DIR := example/app
SERVER_DIR := example/server
SERVER_PORT ?= 9100

.PHONY: help
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@echo '  install:      Install dependencies (cargo fetch + pnpm install)'
	@echo '  dev:          Run the example app on desktop (fast inner loop)'
	@echo '  dev-ios:      Run the example app in the iOS simulator'
	@echo '  dev-android:  Run the example app in the Android emulator'
	@echo '  dev-server:   Run the API server locally on :$(SERVER_PORT)'
	@echo '  check:        Run all checks (fmt, lint, types, test)'
	@echo '  build:        Build server + desktop app bundle'
	@echo '  build-ios:    Build the iOS app (needs Xcode signing)'
	@echo '  testflight:   Build signed for App Store Connect and upload (bin/testflight)'
	@echo '  build-android:Build the Android APK/AAB'
	@echo '  test:         Run Rust tests'
	@echo '  lint:         Run clippy'
	@echo '  fmt:          Format Rust + TS'
	@echo '  fmt-check:    Check formatting'
	@echo '  types:        Type-check TS'
	@echo '  icons:        Regenerate app icons from app-icon.png'
	@echo '  init-ios:     Generate the Xcode project (first time only)'
	@echo '  init-android: Generate the Android project (first time only)'
	@echo '  clean:        Remove build artifacts'

# -- Dev --

# Debug builds of the app point a fresh install at the local server on
# :$(SERVER_PORT) (DEFAULT_SERVER_URL in the app's state.rs); release
# builds default to the deployed ios domain. Settings overrides.

.PHONY: dev
dev: ## Run the example app on desktop
	@pnpm --dir $(APP_DIR) tauri dev $(ARGS)

.PHONY: dev-ios
dev-ios: ## Run the example app in the iOS simulator
	@pnpm --dir $(APP_DIR) tauri ios dev $(ARGS)

.PHONY: dev-android
dev-android: ## Run the example app in the Android emulator
	@pnpm --dir $(APP_DIR) tauri android dev $(ARGS)

.PHONY: dev-server
dev-server: ## Run the API server locally
	@mkdir -p $(SERVER_DIR)/data
	@confit log "Starting the ios server on http://localhost:$(SERVER_PORT) (page + API)..."
	@env SQLITE_DATABASE_URL=sqlite://$(SERVER_DIR)/data/app.db \
		LISTEN_ADDRESS=127.0.0.1:$(SERVER_PORT) \
		POLL_INTERVAL_SECS=$${POLL_INTERVAL_SECS:-120} \
		cargo watch -x "run -p example-server"

# `tauri {ios,android} init` writes default icons into the generated
# project; re-run `icons` afterwards so the brand icon lands in the asset
# catalog too (it writes both src-tauri/icons and gen/*).
.PHONY: init-ios
init-ios: ## Generate the Xcode project under src-tauri/gen/apple
	@pnpm --dir $(APP_DIR) tauri ios init
	@$(MAKE) icons

.PHONY: init-android
init-android: ## Generate the Android project under src-tauri/gen/android
	@pnpm --dir $(APP_DIR) tauri android init
	@$(MAKE) icons

.PHONY: icons
icons: ## Regenerate icons from app-icon.png
	@pnpm --dir $(APP_DIR) tauri icon app-icon.png

# -- Standard targets --

.PHONY: install
install: ## Install dependencies
	cargo fetch
	pnpm install

.PHONY: check
check: fmt-check lint types test ## Run all checks

.PHONY: build
build: ## Build server + desktop app
	cargo build --release -p example-server
	pnpm --dir $(APP_DIR) tauri build $(ARGS)

.PHONY: build-ios
build-ios: ## Build the iOS app
	@pnpm --dir $(APP_DIR) tauri ios build $(ARGS)

.PHONY: testflight
testflight: ## Build signed for App Store Connect and upload to TestFlight
	@../../bin/testflight $(ARGS)

.PHONY: build-android
build-android: ## Build the Android app
	@pnpm --dir $(APP_DIR) tauri android build $(ARGS)

.PHONY: test
test: ## Run all tests
	cargo test --workspace

.PHONY: lint
lint: ## Run clippy linter
	cargo clippy --workspace -- -D warnings

.PHONY: fmt
fmt: ## Format code
	cargo fmt --all
	pnpm -r fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting
	cargo fmt --all -- --check
	pnpm -r fmt-check

.PHONY: types
types: ## Type-check TS
	cargo check --workspace
	pnpm -r typecheck

# Xcode keeps this app's intermediates outside the repo, keyed by project name.
XCODE_DERIVED := $(HOME)/Library/Developer/Xcode/DerivedData/example-app-*

.PHONY: clean
clean: clean-rust clean-js clean-xcode ## Clean all build artifacts (Rust, JS, Xcode)

.PHONY: clean-rust
clean-rust: ## Remove the cargo target dir (desktop + iOS/Android triples)
	cargo clean

.PHONY: clean-js
clean-js: ## Remove node_modules and frontend dist
	rm -rf node_modules std/ts/node_modules std/ts/dist $(APP_DIR)/node_modules $(APP_DIR)/dist

.PHONY: clean-xcode
clean-xcode: ## Remove Xcode DerivedData and the iOS project's build output (keeps gen/apple)
	rm -rf $(XCODE_DERIVED)
	rm -rf $(APP_DIR)/src-tauri/gen/apple/build $(APP_DIR)/src-tauri/gen/apple/Externals/*

.PHONY: clean-gen
clean-gen: ## Remove generated per-platform projects (re-create with init-ios / init-android)
	rm -rf $(APP_DIR)/src-tauri/gen

.PHONY: du
du: ## Show what the build artifacts are costing on disk
	@du -sh target $(APP_DIR)/src-tauri/gen/apple/build $(XCODE_DERIVED) node_modules 2>/dev/null || true
