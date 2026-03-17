#!/usr/bin/env bash
# Start llama-server for Vel's fast utility model on port 8013.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

config_value() {
  local key="$1"
  local config_file="${VEL_CONFIG_PATH:-$ROOT/vel.toml}"
  [[ -f "$config_file" ]] || return 1
  sed -nE "s|^[[:space:]]*$key[[:space:]]*=[[:space:]]*\"([^\"]*)\"[[:space:]]*$|\\1|p" "$config_file" | head -n1
}

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
  echo "Fast model: llama-server not available on PATH and nixpkgs#llama-cpp was not found; fast profile disabled."
  exit 0
fi

CONFIG_MODEL="$(config_value llm_fast_model_path || true)"
MODEL="${VEL_LLM_FAST_MODEL:-${MODEL:-${CONFIG_MODEL:-}}}"
if [[ -z "$MODEL" ]]; then
  echo "Fast model: no model configured. Set VEL_LLM_FAST_MODEL to a .gguf path to enable."
  exit 0
fi
if [[ ! -f "$MODEL" ]]; then
  echo "Fast model: model file not found: $MODEL; fast profile disabled."
  exit 0
fi

HOST="${VEL_LLM_FAST_HOST:-127.0.0.1}"
PORT="${VEL_LLM_FAST_PORT:-8013}"
CTX="${VEL_LLM_FAST_CTX:-8192}"

echo "Starting fast LLM server (port $PORT)..."
exec "$LLAMA_SERVER_BIN" \
  --model "$MODEL" \
  --host "$HOST" \
  --port "$PORT" \
  --jinja \
  --ctx-size "$CTX" \
  --n-gpu-layers 999 \
  --threads "$(nproc)" \
  --parallel 4
