#!/usr/bin/env bash
# Download a small chat model (Qwen2.5-1.5B-Instruct GGUF) for Vel. After this, make dev can use it if VEL_LLM_MODEL is unset (defaults to this path).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WEIGHTS_DIR="${ROOT}/configs/models/weights"
MODEL_NAME="qwen2.5-1.5b-instruct-q4_k_m.gguf"
MODEL_PATH="${WEIGHTS_DIR}/${MODEL_NAME}"
# Hugging Face direct resolve URL (anonymous OK)
URL="https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/${MODEL_NAME}"

mkdir -p "$WEIGHTS_DIR"
if [[ -f "$MODEL_PATH" ]]; then
  echo "Model already present: $MODEL_PATH"
  echo "VEL_LLM_MODEL is not set, so make dev will use this by default."
  exit 0
fi

echo "Downloading ${MODEL_NAME} (~1.1 GB) to ${WEIGHTS_DIR}..."
if command -v curl &>/dev/null; then
  curl -L -o "$MODEL_PATH" "$URL"
elif command -v wget &>/dev/null; then
  wget -O "$MODEL_PATH" "$URL"
else
  echo "Need curl or wget to download."
  exit 1
fi

echo "Done. Model saved at: $MODEL_PATH"
echo "Run 'make dev' (with llama-server on PATH); VEL_LLM_MODEL will default to this path."
