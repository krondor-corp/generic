# DNS

Sets Cloudflare DNS A records for all services, pointing them at the server IP.

```bash
make dns
```

Runs locally (no SSH needed). Reads service domains from `confit.toml` and creates A records via the Cloudflare API.

Idempotent — safe to re-run. Only needed once per service, or when the server IP changes.

Requires `community.general` Ansible collection:

```bash
ansible-galaxy collection install community.general
```
