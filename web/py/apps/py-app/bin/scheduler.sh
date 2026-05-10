#!/bin/bash
# Start the TaskIQ scheduler process

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKIP_DB_SETUP=1 source "$SCRIPT_DIR/env.sh"

echo "[scheduler] Starting TaskIQ scheduler (branch: $(confit resolve worktree.branch), redis: ${REDIS_URL})"
confit --set stage=development run --upper credentials.app -- uv run taskiq scheduler src.tasks.scheduler:scheduler
