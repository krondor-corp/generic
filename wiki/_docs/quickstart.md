---
title: Quickstart
order: 3
---

This guide takes you from a fresh clone to a running deployment in five steps.

## 1. Install tools

```bash
make install
```

Ensure `terraform`, `ansible`, `kamal`, and `op` are also available on your PATH. See [Setup]({{ '/docs/setup/' | relative_url }}) for details.

## 2. Configure

Edit `confit.toml` with your project name, DNS zone, and vault names. Verify everything resolves:

```bash
make validate
```

## 3. Provision infrastructure

```bash
make infra ARGS="init"
make infra ARGS="plan"
make infra ARGS="apply"
```

This creates a DigitalOcean droplet, generates SSH keys stored in Terraform state, and creates Cloudflare DNS records for each service domain.

## 4. Bootstrap the server

```bash
make bootstrap
```

Ansible connects via the Terraform-generated SSH key and installs Docker, creates users, hardens SSH, and configures the firewall.

## 5. Deploy a service

```bash
make kamal ARGS="py setup"
make kamal ARGS="py deploy"
```

Kamal builds the Docker image from `web/py/`, pushes it to Docker Hub, and deploys it behind Traefik with automatic SSL.

## Verify

```bash
make ssh USER=admin
curl https://py.example.com/_status/livez
```

## What's next

- [Terraform]({{ '/docs/terraform/' | relative_url }}) — understand the infrastructure modules
- [Deploying]({{ '/docs/deploying/' | relative_url }}) — add and configure new services
