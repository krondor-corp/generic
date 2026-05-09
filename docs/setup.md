# First-Time Setup

From zero to running services on a DigitalOcean droplet.

## Prerequisites

- [confit](https://github.com/amiller68/confit) — config resolver
- [Terraform](https://developer.hashicorp.com/terraform/install)
- [1Password CLI](https://developer.1password.com/docs/cli/) (`op`) — signed in
- [Kamal](https://kamal-deploy.org/) (`gem install kamal`)
- [Ansible](https://docs.ansible.com/ansible/latest/installation_guide/) (`pip install ansible`)

## 1. Configure confit.toml

Edit `confit.toml` with your project details:

- `project.name` — your project name
- `project.admin_email` — your email (for TFC org)
- `project.tfc_org` — Terraform Cloud organization name
- `vaults.server` — 1Password vault name for server secrets
- `vaults.cloud` — 1Password vault name for cloud provider tokens
- `services.*` — your service definitions

## 2. Set up 1Password vaults

Create the vaults referenced in `confit.toml` and populate them:

### Server vault

| Item | Fields | Notes |
|------|--------|-------|
| `ssh-key-root` | `private_key`, `public_key` | Ed25519 keypair for initial root access |
| `ssh-key-admin` | `private_key`, `public_key` | Ed25519 keypair for admin |
| `ssh-key-kamal` | `private_key`, `public_key` | Ed25519 keypair for deploy user |

### Cloud vault

| Item | Fields | Notes |
|------|--------|-------|
| `DOCKER_HUB_TOKEN` | `credential` | Docker Hub access token (Read & Write) |
| `CLOUDFLARE_API_TOKEN` | `username` (account ID), `credential` (token) | Zone DNS Edit+Read permissions |
| `DIGITALOCEAN_TOKEN` | `credential` | DigitalOcean API token |
| `TFC_TOKEN` | `credential` | Terraform Cloud API token |

## 3. Validate config

```bash
make install
confit validate
```

## 4. Setup Terraform Cloud

```bash
make tfc
```

Creates the organization and workspaces.

## 5. Provision the droplet

```bash
make infra ARGS="init"
make infra ARGS="plan"
make infra ARGS="apply"
```

Creates a DigitalOcean droplet with a firewall (SSH, HTTP, HTTPS) and registers your root SSH key.

## 6. Bootstrap the server

```bash
ANSIBLE_SSH_USER=root make bootstrap
```

On a fresh droplet, root is the only user. After this, root login is disabled and `admin` takes over — subsequent runs are just `make bootstrap`.

Verify: `make ssh USER=admin`

## 7. Set DNS records

```bash
make dns
```

Creates Cloudflare A records for all service domains.

## 8. Deploy services

```bash
make kamal ARGS="my-app setup"
```

Use `setup` for first deploy, `deploy` for subsequent.
