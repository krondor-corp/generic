# SSH Agent & 1Password

`confit ssh` manages a temporary ssh-agent. It resolves a private key from your config, converts it to OpenSSH format, loads it into the agent, runs your command, and kills the agent on exit. The key never touches disk.

```bash
confit ssh --key credentials.ssh.admin.private_key -- ssh admin@server
```

## 1Password IdentityAgent conflict

1Password's desktop app installs an SSH agent and encourages adding this to `~/.ssh/config`:

```
Host *
  IdentityAgent "~/Library/Group Containers/2BUA8C4S2C.com.1password/t/agent.sock"
```

This directive **overrides `SSH_AUTH_SOCK`**, which means SSH ignores any agent you start yourself — including the one `confit ssh` creates. Your key is loaded into the temp agent, but SSH talks to 1Password's agent instead.

**Fix:** remove the `IdentityAgent` line from `~/.ssh/config`. SSH will use `SSH_AUTH_SOCK` as normal, and `confit ssh` will work. If you also want 1Password's agent for interactive use, set it as your default agent via `SSH_AUTH_SOCK` in your shell profile instead of hardcoding it in SSH config.

## Key format conversion

1Password exports Ed25519 keys as PKCS#8 v2 (RFC 8410 OneAsymmetricKey), which includes an optional public key field. OpenSSH's `ssh-add` can't parse this format. `confit ssh` automatically converts keys to OpenSSH format before loading them.

Supported input formats:
- OpenSSH native (`BEGIN OPENSSH PRIVATE KEY`) — passed through
- PKCS#8 PEM (`BEGIN PRIVATE KEY`) — including 1Password's Ed25519 variant
- PKCS#1 RSA PEM (`BEGIN RSA PRIVATE KEY`)
- SEC1 EC PEM (`BEGIN EC PRIVATE KEY`)
- Raw DER

The conversion is handled internally by confit.
