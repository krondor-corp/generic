---
title: Ansible
order: 5
---

Ansible bootstraps a fresh server with everything needed to run containerized services. It is idempotent — safe to re-run at any time.

## Running playbooks

```bash
bin/playbook bootstrap
# or
make bootstrap
```

The `bin/playbook` wrapper handles SSH agent setup automatically via `confit ssh`, using the Terraform-generated private key.

Pass additional Ansible arguments after the playbook name:

```bash
bin/playbook bootstrap --tags users
bin/playbook bootstrap --tags deps,firewall
```

## Bootstrap playbook

`ansible/playbooks/bootstrap.yml` is the main (and currently only) playbook. It runs three task groups:

### Dependencies (`--tags deps`)

Installs base packages, Docker CE, and cloudflared:

- System packages: `curl`, `gnupg`, `ufw`, `tmux`, `git`
- Docker CE with buildx plugin
- Cloudflared (Cloudflare Tunnel client)

### Users (`--tags users`)

Creates system users and configures SSH:

- Creates the admin user with passwordless sudo and docker group membership
- Creates all other users (from `project.users` in confit) with docker group access
- Installs the Terraform-generated SSH public key for every user
- Hardens `sshd_config`: disables root login and password auth, sets a centralized `AuthorizedKeysFile` path

### Firewall (`--tags firewall`)

Configures UFW:

- Rate-limits SSH (port 22)
- Allows HTTP (80) and HTTPS (443)
- Allows high ports (32768–65535) for container networking
- Default deny incoming, allow outgoing

## How config is resolved

The playbook self-serves its variables from confit using Ansible's `lookup('pipe', ...)`:

```yaml
vars:
  admin_user: "{% raw %}{{ lookup('pipe', 'confit resolve project.admin_user') }}{% endraw %}"
  users: "{% raw %}{{ lookup('pipe', 'confit resolve project.users') }}{% endraw %}"
  ssh_public_key: "{% raw %}{{ lookup('pipe', 'confit --set stage=production resolve credentials.ssh.public_key') }}{% endraw %}"
```

No inventory file is needed — the server IP is resolved at runtime and passed as a comma-delimited host list.
