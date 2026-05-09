# Bootstrap

Provisions a fresh server (Ubuntu 24.04). Idempotent — safe to re-run.

```bash
make bootstrap
```

## Tags

| Tag | Task file | What it does |
|-----|-----------|-------------|
| `deps` | `dependencies.yml` | Install packages, Docker CE, cloudflared |
| `users` | `users.yml` | Create users, install SSH keys, harden SSH |
| `firewall` | `firewall.yml` | Configure UFW rules |

```bash
make bootstrap ARGS="--tags users"       # just re-sync users/keys
make bootstrap ARGS="--tags firewall"    # just firewall
```

## SSH user

After bootstrap, root login is disabled — all subsequent runs use admin.

On a fresh server, root is the only user:

```bash
ANSIBLE_SSH_USER=root make bootstrap
```
