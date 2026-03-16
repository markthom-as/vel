#!/usr/bin/env bash
set -euo pipefail

HOST="${HOST:-127.0.0.1}"
PORT="${PORT:-8014}"
MODELS="${MODELS:-gpt-5.4,gpt-5.3-codex}"

exec npx openai-oauth \
  --host "$HOST" \
  --port "$PORT" \
  --models "$MODELS"
