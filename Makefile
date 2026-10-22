.PHONY: help install bootstrap validate kamal ssh services infra tfc wiki clean clean-web clean-mobile clean-wiki du

help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-20s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

install: ## Install confit CLI
	@curl -fsSL https://raw.githubusercontent.com/krondor-corp/confit/main/install.sh | bash

services: ## List configured services
	@confit keys services

tfc: ## Setup Terraform Cloud org and workspaces
	@./bin/tfc setup

infra: ## Run terraform (usage: make infra ARGS="plan" or ARGS="apply")
	@./bin/iac production $(ARGS)

bootstrap: ## Provision the server (packages, users, SSH, firewall)
	@./bin/playbook bootstrap $(ARGS)

validate: ## Validate all config values resolve
	@confit validate

kamal: ## Run Kamal (usage: make kamal ARGS="<service> <command>")
	@./bin/kamal $(ARGS)

ssh: ## SSH into the server (usage: make ssh USER=admin)
	@./bin/ssh $(USER)

wiki: ## Serve wiki locally with live reload
	@$(MAKE) -C wiki dev

# -- Cleaning --
# Each area owns its own clean (web/Makefile fans out to py/ts/rust,
# mobile/Makefile to tauri, which also clears Xcode DerivedData). These
# only remove regenerable build output: never terraform state, data/
# directories, or secrets.

clean: clean-web clean-mobile clean-wiki ## Remove all build artifacts across the repo

clean-web: ## Clean web/ (py, ts, rust)
	@$(MAKE) -C web clean

clean-mobile: ## Clean mobile/ (tauri: cargo target, node_modules, Xcode DerivedData)
	@$(MAKE) -C mobile clean

clean-wiki: ## Clean wiki/ (Jekyll _site)
	@$(MAKE) -C wiki clean

du: ## Show disk used by build artifacts across the repo
	@du -sh web/rust/target web/ts/node_modules web/ts/apps/*/dist web/py/.venv \
		mobile/tauri/target mobile/tauri/node_modules \
		$(HOME)/Library/Developer/Xcode/DerivedData/public-web-api-app-* \
		wiki/_site 2>/dev/null || true
