.PHONY: help install bootstrap validate kamal ssh services infra tfc

help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-20s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

install: ## Install confit CLI (requires cargo)
	@cargo install --git https://github.com/amiller68/confit

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
