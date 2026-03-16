#!/usr/bin/env bash
set -euo pipefail

MODEL="${MODEL:-/models/qwen2.5-coder-14b-instruct-q4.gguf}"
HOST="${HOST:-127.0.0.1}"
PORT="${PORT:-8013}"

exec llama-server \
  --model "$MODEL" \
  --host "$HOST" \
  --port "$PORT" \
  --jinja \
  --ctx-size 8192 \
  --n-gpu-layers 999 \
  --flash-attn \
  --threads "$(nproc)" \
  --parallel 4
