#!/bin/bash
# Shared environment setup for dev server, worker, and scheduler.
# Source this file, then run your process.

WEB_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
PROJECT_ROOT="$(cd "$WEB_ROOT/.." && pwd)"

DB_NAME="$(confit resolve worktree.db_name)"
BACKEND_PORT="$(( 8000 + $(confit resolve worktree.port_offset) ))"
PY_CORE_DIR="$WEB_ROOT/py/packages/py-core"

if [ "${SKIP_DB_SETUP:-}" != "1" ]; then
    "$WEB_ROOT/bin/postgres" ensure_database "$DB_NAME"
    POSTGRES_URL="$(confit resolve accessories.postgres.url)" "$PY_CORE_DIR/bin/db" migrate
fi

export DEV_SERVER_HOST=localhost
export HOST_NAME=http://localhost:${BACKEND_PORT}
export LISTEN_ADDRESS=0.0.0.0
export LISTEN_PORT=${BACKEND_PORT}
export DEBUG=True
export DEV_MODE=True
export LOG_PATH=
export POSTGRES_URL="$(confit resolve accessories.postgres.url)"
export REDIS_URL="$(confit resolve accessories.redis.url)"
export SERVICE_SECRET=veryveryverysecret
