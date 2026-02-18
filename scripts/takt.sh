#!/usr/bin/env bash

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
REPO_ROOT=$(cd "${SCRIPT_DIR}/.." && pwd)

. $HOME/.config/glm/env

export ANTHROPIC_AUTH_TOKEN=${GLM_API_KEY}
export ANTHROPIC_BASE_URL="https://api.z.ai/api/anthropic"
export API_TIMEOUT_MS="3000000"
export ANTHROPIC_DEFAULT_OPUS_MODEL="GLM-5"

export CODEX_HOME=${REPO_ROOT}/.codex

takt "$@"
