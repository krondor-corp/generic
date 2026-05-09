# Ansible Tagging

Playbooks use Ansible tags so you can run specific parts of a setup independently.

## The pattern

When using `include_tasks` with tags, Ansible requires the tag on both the `include_tasks` directive AND via `apply`. Without `apply`, tasks inside the included file won't inherit the tag.

```yaml
# Correct — tags propagate into the included file
- name: Install and configure service
  ansible.builtin.include_tasks:
    file: ../tasks/service-install.yml
    apply:
      tags: [install]
  tags: [install]
```

The outer `tags:` controls whether the include runs. The inner `apply: tags:` ensures every task inside the file also gets tagged.

## Usage

```bash
# Run all tasks
make bootstrap

# Run only tasks tagged "users"
make bootstrap ARGS="--tags users"

# Multiple tags
make bootstrap ARGS="--tags deps,users"
```

See individual playbook docs under [playbooks/](playbooks/) for available tags.
