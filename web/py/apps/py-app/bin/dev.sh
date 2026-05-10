#!/bin/bash
# Start the FastAPI dev server with hot reload

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/env.sh"

echo ""
echo "╔════════════════════════════════════════════════════════╗"
echo "║  Dev Server Starting                                   ║"
echo "╠════════════════════════════════════════════════════════╣"
printf "║  Branch:   %-43s ║\n" "$(confit resolve worktree.branch)"
printf "║  Backend:  %-43s ║\n" "http://localhost:${LISTEN_PORT}"
printf "║  Database: %-43s ║\n" "$(confit resolve worktree.db_name)"
printf "║  Redis:    %-43s ║\n" "${REDIS_URL}"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
confit --set stage=development run --upper credentials.app -- uv run src/__main__.py
