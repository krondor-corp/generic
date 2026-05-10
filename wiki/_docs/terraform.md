---
title: Terraform
order: 4
---

Infrastructure is managed with Terraform, using Terraform Cloud as the state backend.

## Modules

All modules live under `iac/modules/`:

### `digitalocean/ssh_key`

Generates an ED25519 SSH keypair using the `tls_private_key` resource and registers the public key with DigitalOcean. The private key lives only in Terraform state — it is never written to disk or stored in a vault.

**Outputs:** `id`, `fingerprint`, `private_key_openssh` (sensitive), `public_key_openssh`

### `digitalocean/droplet`

Creates a DigitalOcean droplet with the specified size, region, and SSH keys.

**Outputs:** `id`, `urn`, `ipv4_address`

### `digitalocean/project`

Groups resources under a DigitalOcean project for organization.

### `cloudflare/dns`

Creates A records for a list of subdomains pointing at the server IP. Uses `for_each` over the subdomain list.

**Inputs:** `dns_root_zone`, `server_ip`, `subdomains` (list of FQDNs)

## Stages

Each deployment stage has its own Terraform root module under `iac/stages/<stage>/`. Currently there is one stage: `production`.

The production stage wires the modules together:

```hcl
module "ssh_key" { ... }
module "droplet" { ... }      # uses ssh_key.id
module "project" { ... }      # uses droplet.urn
module "dns"     { ... }      # uses droplet.ipv4_address
```

## Running Terraform

Use the `bin/iac` wrapper, which exports all required variables and provider tokens from confit:

```bash
bin/iac production plan
bin/iac production apply
bin/iac production output server_ip
```

Or via Make:

```bash
make infra ARGS="plan"
make infra ARGS="apply"
```

The wrapper automatically resolves:
- `TF_VAR_project_name` and `TF_VAR_dns_root_zone` from `confit.toml`
- `TF_VAR_subdomains` by iterating service domains
- Provider tokens (`DIGITALOCEAN_TOKEN`, `CLOUDFLARE_API_TOKEN`, `TF_TOKEN_app_terraform_io`) from 1Password

## Backend

State is stored in Terraform Cloud. The backend is configured in `iac/stages/production/backend.tf`:

```hcl
terraform {
  cloud {
    organization = "krondor-generic-org"
    workspaces {
      name = "krondor-generic-production"
    }
  }
}
```

## Adding a stage

1. Copy `iac/stages/production/` to `iac/stages/<new-stage>/`
2. Update `backend.tf` with a new workspace name
3. Add a `[vars]` default or pass `--set stage=<new-stage>` to confit
