---
title: Confit
order: 6
---

[confit](https://github.com/amiller68/confit) is a TOML-based config resolver. A single `confit.toml` at the repo root replaces scattered `.env` files, vault references, and service definitions.

## How it works

Resolution happens in two phases:

1. **Interpolation** — `{ref}` tokens are replaced with values from elsewhere in the config. For example, `{project.name}` resolves to the value at `[project] name`.
2. **Provider evaluation** — values with a `scheme://` prefix are passed to the matching provider command. `op://vault/item/field` calls 1Password; `tf://output_name` calls Terraform.

## Config structure

```toml
[providers.op]
cmd = "op read {uri}"

[providers.tf]
cmd = "terraform -chdir=iac/stages/{stage} output -raw {path}"

[project]
name = "krondor-generic"
users = ["admin", "kamal"]
admin_user = "admin"
dns_root_zone = "example.com"

[vars]
stage = "production"

[vaults]
cloud = "cloud-providers"
app = "my-app-{vars.stage}"

[credentials.server]
ip = "tf://server_ip"

[credentials.ssh]
private_key = "secret://tf://ssh_private_key"
public_key = "tf://ssh_public_key"

[credentials.cloud]
docker_hub_token = "secret://op://{vaults.cloud}/DOCKER_HUB_TOKEN/credential"

[services.py]
repo = "krondor-corp/generic"
domain = "py.{project.dns_root_zone}"
port = 8000
```

## Key concepts

### Vars

The `[vars]` section defines variables with defaults. Override them at invocation time:

```bash
confit --set stage=staging resolve credentials.server.ip
```

### Providers

Providers are declared in `[providers]` and invoked when confit sees a matching URI scheme after interpolation:

- `op://` — reads from 1Password via `op read`
- `tf://` — reads Terraform outputs from state

The `{stage}` template variable in the `tf` provider command is populated from `vars.stage`.

### Secret masking

Values prefixed with `secret://` are masked in output by default. Use `--reveal` to show them:

```bash
confit resolve credentials.ssh.private_key              # prints ***
confit resolve --reveal credentials.ssh.private_key     # prints the actual key
```

### SSH agent

`confit ssh` starts a temporary SSH agent, loads a key from config, and re-executes the given command:

```bash
confit ssh --key credentials.ssh.private_key -- ssh admin@server
```

The wrapper sets `CONFIT_SSH=1` to prevent re-entry. All `bin/` scripts use this pattern.

## CLI reference

```
confit resolve <dotted.path>              # Resolve and print a value
confit resolve <dotted.path> --reveal     # Unmask secret:// values
confit resolve <dotted.path> --no-secrets # Stop after interpolation
confit env <section>                      # Print KEY=VALUE pairs
confit env <section> --export             # Prefix with 'export'
confit env <section> --upper              # Uppercase keys
confit keys <section>                     # List leaf keys
confit validate                           # Check all values resolve
confit ssh --key <path> -- <cmd>          # Run command with SSH agent
confit log <message>                      # Styled log output
confit log --err <message>                # Styled error output
```
