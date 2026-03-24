#!/usr/bin/env bash
# Start LLM server (if configured), veld, and the web dev server. Ctrl+C stops all.
set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

config_value() {
  local key="$1"
  local config_file="${VEL_CONFIG_PATH:-$ROOT/vel.toml}"
  [[ -f "$config_file" ]] || return 1
  sed -nE "s|^[[:space:]]*$key[[:space:]]*=[[:space:]]*\"([^\"]*)\"[[:space:]]*$|\\1|p" "$config_file" | head -n1
}

VELD_PID=
LLM_PID=
LLM_FAST_PID=
OPENAI_OAUTH_PID=
cleanup() {
  if [[ -n "$VELD_PID" ]] && kill -0 "$VELD_PID" 2>/dev/null; then
    kill "$VELD_PID" 2>/dev/null || true
    wait "$VELD_PID" 2>/dev/null || true
  fi
  if [[ -n "$LLM_PID" ]] && kill -0 "$LLM_PID" 2>/dev/null; then
    kill "$LLM_PID" 2>/dev/null || true
    wait "$LLM_PID" 2>/dev/null || true
  fi
  if [[ -n "$LLM_FAST_PID" ]] && kill -0 "$LLM_FAST_PID" 2>/dev/null; then
    kill "$LLM_FAST_PID" 2>/dev/null || true
    wait "$LLM_FAST_PID" 2>/dev/null || true
  fi
  if [[ -n "$OPENAI_OAUTH_PID" ]] && kill -0 "$OPENAI_OAUTH_PID" 2>/dev/null; then
    kill "$OPENAI_OAUTH_PID" 2>/dev/null || true
    wait "$OPENAI_OAUTH_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

models_dir="${VEL_MODELS_DIR:-$ROOT/configs/models}"
routing_file="$models_dir/routing.toml"

extract_toml_string() {
  local file="$1"
  local key="$2"
  sed -nE "s|^[[:space:]]*$key[[:space:]]*=[[:space:]]*\"([^\"]*)\"[[:space:]]*$|\\1|p" "$file" | head -n1
}

extract_toml_bool() {
  local file="$1"
  local key="$2"
  sed -nE "s#^[[:space:]]*$key[[:space:]]*=[[:space:]]*(true|false)[[:space:]]*$#\\1#p" "$file" | head -n1
}

profile_file_for_id() {
  local profile_id="$1"
  local file
  shopt -s nullglob
  for file in "$models_dir"/*.toml; do
    if [[ "$(basename "$file")" == "routing.toml" ]]; then
      continue
    fi
    if [[ "$(extract_toml_string "$file" id)" == "$profile_id" ]]; then
      printf '%s\n' "$file"
      return 0
    fi
  done
  return 1
}

ensure_openai_oauth_proxy() {
  [[ -f "$routing_file" ]] || return 0

  local profile_ids=()
  local value
  value="$(extract_toml_string "$routing_file" chat || true)"
  if [[ -n "$value" ]]; then
    profile_ids+=("$value")
  fi
  value="$(extract_toml_string "$routing_file" fallback_remote || true)"
  if [[ -n "$value" ]]; then
    profile_ids+=("$value")
  fi

  if [[ ${#profile_ids[@]} -eq 0 ]]; then
    return 0
  fi

  local seen=""
  local profile_id profile_file provider enabled base_url log_path
  for profile_id in "${profile_ids[@]}"; do
    if [[ " $seen " == *" $profile_id "* ]]; then
      continue
    fi
    seen+=" $profile_id"
    profile_file="$(profile_file_for_id "$profile_id" || true)"
    if [[ -z "$profile_file" ]]; then
      continue
    fi
    provider="$(extract_toml_string "$profile_file" provider || true)"
    enabled="$(extract_toml_bool "$profile_file" enabled || true)"
    if [[ "$provider" != "openai_oauth" || "$enabled" == "false" ]]; then
      continue
    fi
    base_url="$(extract_toml_string "$profile_file" base_url || true)"
    if [[ -z "$base_url" ]]; then
      echo "OpenAI OAuth profile $profile_id is missing base_url in $profile_file"
      exit 1
    fi

    if curl -fsS --max-time 3 "${base_url%/}/models" >/dev/null 2>&1; then
      echo "OpenAI OAuth proxy already ready for $profile_id at $base_url"
      continue
    fi

    mkdir -p "$ROOT/var/logs"
    log_path="$ROOT/var/logs/openai-oauth-${profile_id}.log"
    : >"$log_path"
    echo "Starting OpenAI OAuth proxy for $profile_id at $base_url..."
    "$ROOT/scripts/openai-oauth.sh" run --base-url "$base_url" >"$log_path" 2>&1 &
    OPENAI_OAUTH_PID=$!

    local ready=0
    for ((i = 1; i <= 120; i++)); do
      if curl -fsS --max-time 3 "${base_url%/}/models" >/dev/null 2>&1; then
        ready=1
        break
      fi
      if ! kill -0 "$OPENAI_OAUTH_PID" 2>/dev/null; then
        echo "OpenAI OAuth proxy exited unexpectedly for $profile_id. Log: $log_path"
        tail -n 20 "$log_path" || true
        exit 1
      fi
      sleep 1
    done

    if [[ "$ready" -ne 1 ]]; then
      echo "OpenAI OAuth proxy did not become ready for $profile_id at $base_url. Log: $log_path"
      tail -n 20 "$log_path" || true
      kill "$OPENAI_OAUTH_PID" 2>/dev/null || true
      exit 1
    fi

    echo "OpenAI OAuth proxy ready for $profile_id."
  done
}

if [[ -z "${VEL_LLM_MODEL:-}" ]]; then
  CONFIG_LLM_MODEL="$(config_value llm_model_path || true)"
  if [[ -n "$CONFIG_LLM_MODEL" ]]; then
    export VEL_LLM_MODEL="$CONFIG_LLM_MODEL"
  elif [[ -f "$ROOT/configs/models/weights/qwen2.5-1.5b-instruct-q4_k_m.gguf" ]]; then
    export VEL_LLM_MODEL="$ROOT/configs/models/weights/qwen2.5-1.5b-instruct-q4_k_m.gguf"
  fi
fi

if [[ -z "${VEL_LLM_FAST_MODEL:-}" ]]; then
  CONFIG_LLM_FAST_MODEL="$(config_value llm_fast_model_path || true)"
  if [[ -n "$CONFIG_LLM_FAST_MODEL" ]]; then
    export VEL_LLM_FAST_MODEL="$CONFIG_LLM_FAST_MODEL"
  fi
fi

# Start primary LLM server (port 8012) if VEL_LLM_MODEL is set.
if [[ -n "${VEL_LLM_MODEL:-}" ]]; then
  echo "Starting primary LLM server..."
  "$ROOT/scripts/llm-server.sh" &
  LLM_PID=$!
  sleep 2
else
  echo "Primary model: set VEL_LLM_MODEL to a .gguf path to enable assistant replies."
fi

# Start fast LLM server (port 8013) if VEL_LLM_FAST_MODEL is set.
if [[ -n "${VEL_LLM_FAST_MODEL:-}" ]]; then
  echo "Starting fast LLM server..."
  "$ROOT/scripts/llm-server-fast.sh" &
  LLM_FAST_PID=$!
  sleep 2
fi

ensure_openai_oauth_proxy

# Default veld bind (vel-config); client must use same host:port via VITE_API_URL
API_URL="${VITE_API_URL:-http://localhost:4130}"
# Parse port for wait (e.g. 4130 from http://localhost:4130)
PORT="${API_URL##*:}"
PORT="${PORT%%/*}"

echo "Starting veld (API at $API_URL)..."
"$ROOT/scripts/dev-api.sh" &
VELD_PID=$!

echo "Waiting for veld on port $PORT..."
READY_TIMEOUT_SECONDS="${VELD_READY_TIMEOUT_SECONDS:-120}"
READY_POLL_INTERVAL_SECONDS="${VELD_READY_POLL_INTERVAL_SECONDS:-0.5}"
MAX_POLLS="$(awk "BEGIN { printf \"%d\", ($READY_TIMEOUT_SECONDS / $READY_POLL_INTERVAL_SECONDS) + 0.999 }")"
for ((i = 1; i <= MAX_POLLS; i++)); do
  if curl -s -o /dev/null -w "%{http_code}" "http://127.0.0.1:$PORT/v1/health" 2>/dev/null | grep -q 200; then
    break
  fi
  if ! kill -0 "$VELD_PID" 2>/dev/null; then
    echo "veld exited unexpectedly"
    exit 1
  fi
  sleep "$READY_POLL_INTERVAL_SECONDS"
done
if ! curl -s -o /dev/null "http://127.0.0.1:$PORT/v1/health" 2>/dev/null; then
  echo "veld did not become ready in time (${READY_TIMEOUT_SECONDS}s)"
  kill "$VELD_PID" 2>/dev/null || true
  exit 1
fi
echo "veld ready."

echo "Starting web dev server (VITE_API_URL=$API_URL)..."
cd clients/web && exec npm run dev
