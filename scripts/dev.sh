#!/usr/bin/env bash
# Start LLM server (if configured), veld, and the web dev server. Ctrl+C stops all.
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VELD_PID=
LLM_PID=
cleanup() {
  if [[ -n "$VELD_PID" ]] && kill -0 "$VELD_PID" 2>/dev/null; then
    kill "$VELD_PID" 2>/dev/null || true
    wait "$VELD_PID" 2>/dev/null || true
  fi
  if [[ -n "$LLM_PID" ]] && kill -0 "$LLM_PID" 2>/dev/null; then
    kill "$LLM_PID" 2>/dev/null || true
    wait "$LLM_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

# Default chat model path if downloaded by scripts/download-chat-model.sh
if [[ -z "${VEL_LLM_MODEL:-}" ]] && [[ -f "$ROOT/configs/models/weights/qwen2.5-1.5b-instruct-q4_k_m.gguf" ]]; then
  export VEL_LLM_MODEL="$ROOT/configs/models/weights/qwen2.5-1.5b-instruct-q4_k_m.gguf"
fi

# Start LLM server for chat (port 8012) if VEL_LLM_MODEL is set and llama-server is available.
if [[ -n "${VEL_LLM_MODEL:-}" ]] && command -v llama-server &>/dev/null; then
  echo "Starting LLM server for chat..."
  "$ROOT/scripts/llm-server.sh" &
  LLM_PID=$!
  sleep 2
else
  echo "Chat: set VEL_LLM_MODEL to a .gguf path and have llama-server in PATH to enable assistant replies."
fi

# Default veld bind (vel-config); client must use same host:port via VITE_API_URL
API_URL="${VITE_API_URL:-http://localhost:4130}"
# Parse port for wait (e.g. 4130 from http://localhost:4130)
PORT="${API_URL##*:}"
PORT="${PORT%%/*}"

echo "Starting veld (API at $API_URL)..."
cargo run -p veld &
VELD_PID=$!

echo "Waiting for veld on port $PORT..."
for i in {1..30}; do
  if curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORT/v1/health" 2>/dev/null | grep -q 200; then
    break
  fi
  if ! kill -0 "$VELD_PID" 2>/dev/null; then
    echo "veld exited unexpectedly"
    exit 1
  fi
  sleep 0.5
done
if ! curl -s -o /dev/null "http://127.0.0.1:$PORT/v1/health" 2>/dev/null; then
  echo "veld did not become ready in time"
  kill "$VELD_PID" 2>/dev/null || true
  exit 1
fi
echo "veld ready."

echo "Starting web dev server (VITE_API_URL=$API_URL)..."
cd clients/web && exec npm run dev
