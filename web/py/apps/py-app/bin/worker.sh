#!/bin/bash
# Start the TaskIQ worker process

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKIP_DB_SETUP=1 source "$SCRIPT_DIR/env.sh"

echo "[worker] Starting TaskIQ worker (branch: $(confit resolve worktree.branch), redis: ${REDIS_URL})"
confit --set stage=development run --upper credentials.app -- uv run taskiq worker src.tasks:broker --fs-discover --reload
