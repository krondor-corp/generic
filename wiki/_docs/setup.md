---
title: Setup
order: 2
---

## Prerequisites

You need the following tools installed locally:

| Tool | Purpose | Install |
|------|---------|---------|
| [confit](https://github.com/amiller68/confit) | Config resolution | `cargo install --git https://github.com/amiller68/confit` |
| [Terraform](https://developer.hashicorp.com/terraform/install) | Infrastructure provisioning | `brew install terraform` |
| [Ansible](https://docs.ansible.com/ansible/latest/installation_guide/) | Server bootstrap | `brew install ansible` |
| [Kamal](https://kamal-deploy.org/) | Container deployment | `gem install kamal` |
| [1Password CLI](https://developer.1password.com/docs/cli/) | Secret resolution | `brew install 1password-cli` |

## Installation

Clone the repo and install confit:

```bash
gh repo create my-project --template krondor-corp/generic
cd my-project
make install
```

`make install` builds and installs the `confit` CLI from source.

## Configuration

All configuration lives in `confit.toml` at the repository root. At minimum, update these values for your project:

```toml
[project]
name = "my-project"
admin_email = "you@example.com"
dns_root_zone = "example.com"
tfc_org = "my-tfc-org"

[vaults]
cloud = "my-cloud-vault"
app = "my-app-{vars.stage}"
```

The `[vaults]` section references 1Password vault names. The `[providers]` section defines how `op://` and `tf://` URIs are resolved — you shouldn't need to change these unless you swap secret backends.

## Terraform Cloud

Create a Terraform Cloud organization and workspace:

```bash
make tfc
```

This runs `bin/tfc setup`, which creates the org and workspace defined in `confit.toml`. Update `iac/stages/production/backend.tf` to match your org and workspace names.

## 1Password

The template expects two 1Password vaults:

- **cloud** — cloud provider tokens (DigitalOcean, Cloudflare, Docker Hub, Terraform Cloud)
- **app** — application secrets (OAuth credentials, API keys)

Create the required items in each vault. See [confit]({{ '/docs/confit/' | relative_url }}) for the full list of referenced paths.
