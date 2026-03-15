#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

# mise で管理されたツール（node 等）を PATH に追加
eval "$(mise env 2>/dev/null)" || true

# ACCOUNT environment variable takes precedence over first positional argument.
# If ACCOUNT is already set, $1 is treated as a takt argument, not an account name.
if [[ -z "${ACCOUNT:-}" ]]; then
  ACCOUNT="${1:-corporate}"
  shift 2>/dev/null || true
fi

# Short aliases
case "$ACCOUNT" in
p) ACCOUNT=personal ;;
c) ACCOUNT=corporate ;;
z) ACCOUNT=zai ;;
esac

CLAUDE_WRAPPER="${SCRIPT_DIR}/run-claude-${ACCOUNT}.sh"
CODEX_WRAPPER="${SCRIPT_DIR}/run-codex-${ACCOUNT}.sh"

if [[ ! -f "$CLAUDE_WRAPPER" && ! -f "$CODEX_WRAPPER" ]]; then
  echo "[ERROR] Unknown account: $ACCOUNT" >&2
  echo "[INFO] Available: $(ls "$SCRIPT_DIR"/run-claude-*.sh 2>/dev/null | sed 's/.*run-claude-//;s/\.sh//' | paste -sd', ')" >&2
  echo "[INFO] Short aliases: p=personal, c=corporate, z=zai" >&2
  exit 1
fi

[[ -f "$CLAUDE_WRAPPER" ]] && export TAKT_CLAUDE_CLI_PATH="$CLAUDE_WRAPPER"
[[ -f "$CODEX_WRAPPER" ]] && export TAKT_CODEX_CLI_PATH="$CODEX_WRAPPER"

echo "ACCOUNT=${ACCOUNT}"

exec takt "$@"
