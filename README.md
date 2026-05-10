# generic

A monorepo template for building and deploying web services to a single server. Application source, infrastructure, and deployment config all live together.

**[Docs](https://generic.krondor.org/wiki/)** · **[Live Demo](https://generic.krondor.org)**

## Services

| Service | Stack | Port |
|---------|-------|------|
| `py` | FastAPI, PostgreSQL, Redis, TaskIQ, pydantic-ai | 8000 |
| `rust` | Axum, SQLite, Askama, HTMX, Apalis | 3000 |
| `ts-web` | React, Vite, pnpm, turbo | 3000 |

## Infrastructure

| Layer | Tool |
|-------|------|
| Config | [confit](https://confit.krondor.org) — single `confit.toml` wires secrets + values across all layers |
| Provision | Terraform — DigitalOcean droplet, SSH keys, Cloudflare DNS |
| Bootstrap | Ansible — users, SSH hardening, Docker, firewall |
| Deploy | Kamal — container builds, zero-downtime rolling deploys, Traefik proxy, SSL |

## Quick start

```bash
# Install confit
curl -fsSL https://raw.githubusercontent.com/krondor-corp/confit/main/install.sh | bash

# Edit config
vim confit.toml

# Provision infrastructure
make tfc
make infra ARGS="apply"

# Bootstrap the server
make bootstrap

# Deploy
make kamal ARGS="py setup"
make kamal ARGS="py deploy"
```

## Development

Each service has its own Makefile with the same interface:

```bash
cd web/py    # or web/rust, web/ts
make install
make dev
make check   # fmt + lint + types + test
```

## Project structure

```
confit.toml                  # Central config — credentials, services, project metadata
Makefile                     # Top-level targets
bin/                         # Wrapper scripts (iac, kamal, playbook, ssh)
iac/                         # Terraform modules and stages
ansible/                     # Playbooks and tasks
config/deploy/               # Kamal deploy configs (ERB + confit)
web/
  py/                        # Python service (FastAPI + PostgreSQL + Redis)
  rust/                      # Rust service (Axum + SQLite + HTMX)
  ts/                        # TypeScript service (React + Vite + turbo)
wiki/                        # Documentation site (Jekyll, GitHub Pages)
```

## Make targets

```bash
make install                       # Install confit
make infra ARGS="plan|apply"       # Run terraform
make bootstrap                     # Provision the server
make kamal ARGS="<service> <cmd>"  # Deploy / manage services
make ssh USER=admin                # SSH into the server
make services                      # List configured services
make validate                      # Check all config values resolve
make wiki                          # Serve docs locally
```

## How config works

Everything flows through `confit.toml`:

```
confit.toml
    ├── op://  → 1Password (API tokens, credentials)
    ├── tf://  → Terraform outputs (server IP, SSH keys)
    └── {ref}  → interpolation between config values
```

Terraform, Ansible, and Kamal all read from confit. Wrapper scripts in `bin/` handle SSH agent setup, secret injection, and provider resolution — never run the underlying tools directly.

## Adding a service

1. Add source under `web/<name>/` with a `Dockerfile` and `Makefile`
2. Add `[services.<name>]` to `confit.toml` with `repo`, `domain`, `port`
3. Create `config/deploy/<name>.yml` (use existing configs as a template)
4. `make infra ARGS="apply"` to create the DNS record
5. `make kamal ARGS="<name> setup && <name> deploy"`
