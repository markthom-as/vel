#!/usr/bin/env bash
set -euo pipefail

MODEL="${MODEL:-/models/qwen3-coder-30b-a3b-instruct-q4.gguf}"
HOST="${HOST:-127.0.0.1}"
PORT="${PORT:-8012}"

exec llama-server \
  --model "$MODEL" \
  --host "$HOST" \
  --port "$PORT" \
  --jinja \
  --ctx-size 16384 \
  --n-gpu-layers 999 \
  --flash-attn \
  --cache-type-k q8_0 \
  --cache-type-v q8_0 \
  --threads "$(nproc)" \
  --parallel 2
