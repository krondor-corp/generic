# generic-iac

Infrastructure template for deploying services to a DigitalOcean droplet.

**Stack:** [confit](https://github.com/amiller68/confit) + Terraform + Ansible + [Kamal](https://kamal-deploy.org/)

| Layer | Tool | What |
|-------|------|------|
| Provision | Terraform | DigitalOcean droplet + firewall |
| Configure | Ansible | Users, SSH, Docker, firewall (UFW) |
| Deploy | Kamal | Container builds, rolling deploys, Traefik proxy |
| Config | confit | Single `confit.toml` wires secrets + values across all layers |

## Prerequisites

- [confit](https://github.com/amiller68/confit) — config resolver
- [Terraform](https://developer.hashicorp.com/terraform/install) — infrastructure provisioning
- [Kamal](https://kamal-deploy.org/) (`gem install kamal`)
- [1Password CLI](https://developer.1password.com/docs/cli/) (`op`)
- [Ansible](https://docs.ansible.com/ansible/latest/installation_guide/) (`pip install ansible`)

## Quick Start

```bash
# 1. Edit confit.toml with your project details
vim confit.toml

# 2. Set up 1Password vaults (see docs/setup.md)

# 3. Setup Terraform Cloud
make tfc

# 4. Provision the droplet
make infra ARGS="plan"
make infra ARGS="apply"

# 5. Bootstrap the server
ANSIBLE_SSH_USER=root make bootstrap

# 6. Set DNS records
make dns

# 7. Deploy your service
make kamal ARGS="my-app setup"
```

## Quick Reference

```bash
# Infrastructure
make tfc                               # Setup Terraform Cloud org/workspaces
make infra ARGS="plan"                 # Plan infrastructure changes
make infra ARGS="apply"                # Apply infrastructure changes

# Server provisioning
make bootstrap                         # Provision the server
make bootstrap ARGS="--tags users"     # Just re-sync users/keys

# DNS
make dns                               # Set A records for services

# Service management
make services                          # List all services
make kamal ARGS="my-app deploy"        # Deploy a service
make kamal ARGS="my-app setup"         # First-time deploy

# confit CLI
confit resolve project.name                            # Resolve a config value
confit --set stage=production resolve credentials.server.ip  # Resolve with TF provider
confit keys services                                   # List service names
confit show services.my-app.env                        # Print KEY=VALUE pairs
confit show --yaml --reveal credentials.ssh            # YAML output, unmasked
confit ssh --key credentials.ssh.admin.private_key -- ssh admin@server

# Utilities
make validate                          # Check all config resolves
make ssh USER=admin                    # SSH into the server
```

## Project Structure

```
confit.toml                  # All config — credentials, services, project metadata
iac/
  modules/digitalocean/      # Reusable TF modules (droplet, project)
  stages/production/         # Production stage (droplet + firewall)
bin/
  playbook                   # Unified playbook runner (SSH agent + ansible-playbook)
  kamal                      # Kamal deploy wrapper
  ssh                        # SSH into the server
  iac                        # Terraform wrapper (injects secrets from confit)
  tfc                        # Terraform Cloud workspace manager
ansible/
  playbooks/*.yml            # Ansible playbook entrypoints
  tasks/*.yml                # Ansible task files
config/deploy/*.yml          # Kamal deploy configs (ERB templates using confit)
.kamal/secrets               # Kamal secrets (resolved via confit at deploy time)
docs/                        # Documentation
```

## How It Works

Everything flows through `confit.toml`:

1. **Terraform** provisions the droplet. `bin/iac` exports secrets from confit as `TF_VAR_*` and runs terraform. The server IP is stored as a TF output.

2. **Ansible** bootstraps the server. Playbooks self-serve their vars via `lookup('pipe', 'confit ...')`. `bin/playbook` handles the SSH agent and inventory.

3. **Kamal** deploys containers. Deploy configs use ERB to call `confit resolve` for dynamic values. `.kamal/secrets` resolves secrets at deploy time.

4. **confit** ties it together. The `tf://` provider reads terraform outputs, `op://` reads 1Password, `secret://` masks sensitive values.

```
confit.toml
    ├── op:// → 1Password (SSH keys, API tokens)
    ├── tf:// → Terraform outputs (server IP)
    └── {ref} → interpolation (vault names, project config)
```

## Documentation

| Doc | What |
|-----|------|
| [Setup](docs/setup.md) | First-time setup from zero |
| [Tagging](docs/tagging.md) | Ansible tag pattern for selective runs |
| [SSH Agent](docs/ssh-agent.md) | How confit ssh works, 1Password key format handling |
| [Bootstrap](docs/playbooks/bootstrap.md) | Server provisioning |
| [DNS](docs/playbooks/dns.md) | Cloudflare DNS records |

## Contributing

### Adding a service

1. Add to `confit.toml`:
   ```toml
   [services.my-new-app]
   repo = "user/repo"
   domain = "app.example.com"
   port = 3000

   [services.my-new-app.env]
   NODE_ENV = "production"
   ```

2. Create `services/my-new-app/Dockerfile`

3. Create `config/deploy/my-new-app.yml` (use ERB + `confit resolve`)

4. Add secrets to `.kamal/secrets` if needed

5. `make dns && make kamal ARGS="my-new-app setup"`

### Adding a playbook

1. Create `ansible/tasks/<name>.yml` and `ansible/playbooks/<name>.yml`
2. Playbooks self-serve vars: `"{{ lookup('pipe', 'confit resolve ...') }}"`
3. Add a Makefile target: `@./bin/playbook <name> $(ARGS)`
4. Add docs at `docs/playbooks/<name>.md`
