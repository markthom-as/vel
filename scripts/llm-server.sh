#!/usr/bin/env bash
# Start llama-server for Vel chat (configs/models: local-qwen3-coder on port 8012).
# Set VEL_LLM_MODEL to your .gguf model path. If unset or llama-server missing, exits 0 so make dev still runs.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v llama-server &>/dev/null; then
  echo "Chat: llama-server not in PATH; assistant replies disabled. Install llama.cpp and add llama-server to PATH to enable."
  exit 0
fi

MODEL="${VEL_LLM_MODEL:-${MODEL:-}}"
if [[ -z "$MODEL" ]]; then
  echo "Chat: VEL_LLM_MODEL not set; assistant replies disabled. Set VEL_LLM_MODEL to a .gguf path (e.g. configs/models/qwen3-coder-7b.gguf) to enable."
  exit 0
fi
if [[ ! -f "$MODEL" ]]; then
  echo "Chat: model file not found: $MODEL; assistant replies disabled."
  exit 0
fi

HOST="${VEL_LLM_HOST:-127.0.0.1}"
PORT="${VEL_LLM_PORT:-8012}"
CTX="${VEL_LLM_CTX:-8192}"

echo "Starting LLM server (port $PORT) for chat..."
exec llama-server \
  --model "$MODEL" \
  --host "$HOST" \
  --port "$PORT" \
  --jinja \
  --ctx-size "$CTX" \
  --n-gpu-layers 999 \
  --threads "$(nproc)" \
  --parallel 2
