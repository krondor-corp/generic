---
title: Deploying
order: 7
---

Services are deployed with [Kamal](https://kamal-deploy.org/), which builds Docker containers and rolls them out behind a Traefik reverse proxy with automatic SSL.

## Deploy a service

```bash
bin/kamal py deploy
# or
make kamal ARGS="py deploy"
```

First-time setup for a new service:

```bash
bin/kamal py setup
bin/kamal py deploy
```

## How it works

1. `bin/kamal` wraps itself in `confit ssh` to load the Terraform-generated deploy key
2. Kamal reads the service config from `config/deploy/<service>.yml`
3. Secrets are injected from `.kamal/secrets`, which calls `confit resolve --reveal`
4. Kamal builds the Docker image (context: `web/`, dockerfile: `web/<service>/Dockerfile`)
5. The image is pushed to Docker Hub and deployed with zero-downtime rolling updates

## Service configs

Each service has a Kamal config at `config/deploy/<service>.yml`. These are ERB templates that call confit for dynamic values:

```yaml
service: <%%= `confit resolve project.name`.strip %>-py
image: <%%= `confit resolve services.py.repo`.strip %>-py

servers:
  web:
    - <%%= `confit --set stage=production resolve credentials.server.ip`.strip %>

proxy:
  ssl: true
  host: <%%= `confit resolve services.py.domain`.strip %>
  app_port: <%%= `confit resolve services.py.port`.strip %>
```

Environment variables are defined directly in the Kamal config — not in `confit.toml`.

## Secrets

`.kamal/secrets` resolves secrets via confit and exports them for Kamal:

```bash
DOCKER_HUB_TOKEN=$(confit resolve --reveal credentials.cloud.docker_hub_token)
```

Reference these in the Kamal config under `env.secret`.

## Adding a new service

1. Add the service source under `web/<name>/` with a `Dockerfile`
2. Add a `[services.<name>]` entry in `confit.toml` with `repo`, `domain`, and `port`
3. Create `config/deploy/<name>.yml` using an existing config as a template
4. Run `make infra ARGS="apply"` to create the DNS record (the subdomain is picked up automatically from `services.<name>.domain`)
5. Deploy: `bin/kamal <name> setup && bin/kamal <name> deploy`

## Available services

| Service | Domain | Port | Stack |
|---------|--------|------|-------|
| py | `py.<zone>` | 8000 | Python (FastAPI) |
| rust | `rust.<zone>` | 3000 | Rust (Axum) |
| ts-web | `ts.<zone>` | 3000 | TypeScript (Next.js) |
