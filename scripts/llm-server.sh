#!/usr/bin/env bash
# Start llama-server for Vel chat (configs/models: local-qwen3-coder on port 8012).
# Defaults to the repo's downloaded GGUF if present so local dev works in a fresh shell.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

LLAMA_SERVER_BIN="${LLAMA_SERVER_BIN:-}"
if [[ -z "$LLAMA_SERVER_BIN" ]] && command -v llama-server &>/dev/null; then
  LLAMA_SERVER_BIN="$(command -v llama-server)"
fi
if [[ -z "$LLAMA_SERVER_BIN" ]] && command -v nix &>/dev/null; then
  NIX_LLAMA_CPP_PATH="$(nix build --no-link --print-out-paths nixpkgs#llama-cpp 2>/dev/null || true)"
  if [[ -n "$NIX_LLAMA_CPP_PATH" ]] && [[ -x "$NIX_LLAMA_CPP_PATH/bin/llama-server" ]]; then
    LLAMA_SERVER_BIN="$NIX_LLAMA_CPP_PATH/bin/llama-server"
  fi
fi
if [[ -z "$LLAMA_SERVER_BIN" ]]; then
  echo "Chat: llama-server not available on PATH and nixpkgs#llama-cpp was not found; assistant replies disabled."
  exit 0
fi

DEFAULT_MODEL="$ROOT/configs/models/weights/qwen2.5-1.5b-instruct-q4_k_m.gguf"
MODEL="${VEL_LLM_MODEL:-${MODEL:-}}"
if [[ -z "$MODEL" ]] && [[ -f "$DEFAULT_MODEL" ]]; then
  MODEL="$DEFAULT_MODEL"
fi
if [[ -z "$MODEL" ]]; then
  echo "Chat: no model configured and no default GGUF found; assistant replies disabled. Set VEL_LLM_MODEL to a .gguf path to enable."
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
exec "$LLAMA_SERVER_BIN" \
  --model "$MODEL" \
  --host "$HOST" \
  --port "$PORT" \
  --jinja \
  --ctx-size "$CTX" \
  --n-gpu-layers 999 \
  --threads "$(nproc)" \
  --parallel 2
