---
title: Overview
order: 1
---

**generic** is a monorepo template for deploying web services to a DigitalOcean droplet. It combines infrastructure-as-code, configuration management, and application source in a single repository.

## Philosophy

All you really need to deploy a modern web app is a decent DNS provider, a place to run a container, and a nice proxy around it. Cloud providers want you to learn their ALBs, their container services, their secret managers — knowledge that doesn't transfer when you switch providers or outgrow their free tier.

Generic uses commodity tools instead. Cloudflare for DNS, any Linux VM for compute, Traefik for reverse proxying with automatic SSL. You understand every layer, and nothing is provider-specific except the VM itself (which Terraform makes trivially swappable).

Until you're orchestrating dozens of containers or handling enough traffic to need a dedicated load balancer, a single server with Docker and Kamal gets you zero-downtime deploys, rolling updates, SSL, and environment management — without stitching together Nginx configs, docker-compose files, and deployment scripts by hand.

## What's in the box

| Layer | Tool | Purpose |
|-------|------|---------|
| Config | [confit]({{ '/docs/confit/' | relative_url }}) | Single TOML file with interpolation and secret resolution |
| Infra | [Terraform]({{ '/docs/terraform/' | relative_url }}) | DigitalOcean droplet, SSH keys, Cloudflare DNS |
| Bootstrap | [Ansible]({{ '/docs/ansible/' | relative_url }}) | Users, SSH hardening, Docker, firewall |
| Deploy | [Kamal]({{ '/docs/deploying/' | relative_url }}) | Zero-downtime container deploys behind Traefik |

## Repository layout

```
├── confit.toml              # Central config (all values, secrets, services)
├── Makefile                 # Top-level targets
├── bin/                     # Wrapper scripts (iac, kamal, playbook, ssh)
├── iac/
│   ├── modules/             # Reusable Terraform modules
│   └── stages/production/   # Stage-specific Terraform root
├── ansible/
│   ├── playbooks/           # Entrypoint playbooks
│   └── tasks/               # Reusable task files
├── config/deploy/           # Kamal deploy configs (per service)
├── web/                     # Application source code
│   ├── py/                  # Python service
│   ├── rust/                # Rust service
│   └── ts/                  # TypeScript service
└── wiki/                    # This documentation site
```

## Flow

1. **Provision** — `make infra ARGS="apply"` creates the droplet, generates SSH keys, and sets DNS records.
2. **Bootstrap** — `make bootstrap` installs Docker, creates users, hardens SSH, and configures the firewall.
3. **Deploy** — `make kamal ARGS="py deploy"` builds a Docker image from `web/`, pushes it, and rolls it out with zero downtime.

All three steps pull their configuration from `confit.toml`. Secrets are resolved on-the-fly from 1Password (`op://`) or Terraform state (`tf://`).

## Next steps

- [Setup]({{ '/docs/setup/' | relative_url }}) — prerequisites and installation
- [Quickstart]({{ '/docs/quickstart/' | relative_url }}) — provision and deploy in five minutes
