#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

config_value() {
  local key="$1"
  local config_file="${VEL_CONFIG_PATH:-$ROOT/vel.toml}"
  [[ -f "$config_file" ]] || return 1
  sed -nE "s|^[[:space:]]*$key[[:space:]]*=[[:space:]]*\"([^\"]*)\"[[:space:]]*$|\\1|p" "$config_file" | head -n1
}

resolve_path() {
  local path="$1"
  if [[ "$path" = /* ]]; then
    printf '%s\n' "$path"
  else
    printf '%s/%s\n' "$ROOT" "$path"
  fi
}

PRIMARY_RAW="${VEL_LLM_MODEL:-$(config_value llm_model_path || true)}"
FAST_RAW="${VEL_LLM_FAST_MODEL:-$(config_value llm_fast_model_path || true)}"

PRIMARY_PATH="$(resolve_path "${PRIMARY_RAW:-}")"
FAST_PATH="$(resolve_path "${FAST_RAW:-}")"

echo "Config file: ${VEL_CONFIG_PATH:-$ROOT/vel.toml}"
echo "Primary model path: ${PRIMARY_RAW:-<unset>}"
echo "Fast model path: ${FAST_RAW:-<unset>}"
echo

for label in "Primary:$PRIMARY_PATH" "Fast:$FAST_PATH"; do
  name="${label%%:*}"
  path="${label#*:}"
  if [[ -z "$path" || "$path" = "$ROOT/" ]]; then
    echo "$name model: unset"
    continue
  fi
  if [[ -f "$path" ]]; then
    size="$(ls -lh "$path" | awk '{print $5}')"
    echo "$name model: present ($size) at $path"
  else
    echo "$name model: missing at $path"
  fi
done

echo
if command -v llama-server >/dev/null 2>&1; then
  echo "llama-server: $(command -v llama-server)"
  llama-server --version 2>/dev/null || true
else
  echo "llama-server: not on PATH"
fi

echo
if command -v nvidia-smi >/dev/null 2>&1; then
  if nvidia-smi --query-gpu=name,memory.total,driver_version --format=csv,noheader >/tmp/vel-nvidia-smi.out 2>/tmp/vel-nvidia-smi.err; then
    echo "GPU:"
    cat /tmp/vel-nvidia-smi.out
  else
    echo "GPU: nvidia-smi failed"
    cat /tmp/vel-nvidia-smi.err
  fi
else
  echo "GPU: nvidia-smi not found"
fi

echo
if ls /dev/nvidia* >/dev/null 2>&1; then
  echo "NVIDIA device nodes:"
  ls -l /dev/nvidia*
else
  echo "NVIDIA device nodes: none found"
fi

echo
MODELS_DIR="${VEL_MODELS_DIR:-$ROOT/configs/models}"
ROUTING_FILE="$MODELS_DIR/routing.toml"
if [[ -f "$ROUTING_FILE" ]]; then
  chat_profile="$(sed -nE 's|^[[:space:]]*chat[[:space:]]*=[[:space:]]*"([^"]*)"[[:space:]]*$|\1|p' "$ROUTING_FILE" | head -n1)"
  fallback_profile="$(sed -nE 's|^[[:space:]]*fallback_remote[[:space:]]*=[[:space:]]*"([^"]*)"[[:space:]]*$|\1|p' "$ROUTING_FILE" | head -n1)"
  echo "Routing chat profile: ${chat_profile:-<unset>}"
  echo "Routing fallback profile: ${fallback_profile:-<unset>}"
fi

oauth_profiles=0
shopt -s nullglob
for profile in "$MODELS_DIR"/*.toml; do
  if [[ "$(basename "$profile")" == "routing.toml" ]]; then
    continue
  fi
  provider="$(sed -nE 's|^[[:space:]]*provider[[:space:]]*=[[:space:]]*"([^"]*)"[[:space:]]*$|\1|p' "$profile" | head -n1)"
  enabled="$(sed -nE 's#^[[:space:]]*enabled[[:space:]]*=[[:space:]]*(true|false)[[:space:]]*$#\1#p' "$profile" | head -n1)"
  if [[ "$provider" != "openai_oauth" || "$enabled" == "false" ]]; then
    continue
  fi
  oauth_profiles=1
  base_url="$(sed -nE 's|^[[:space:]]*base_url[[:space:]]*=[[:space:]]*"([^"]*)"[[:space:]]*$|\1|p' "$profile" | head -n1)"
  profile_id="$(sed -nE 's|^[[:space:]]*id[[:space:]]*=[[:space:]]*"([^"]*)"[[:space:]]*$|\1|p' "$profile" | head -n1)"
  echo
  echo "OpenAI OAuth profile: ${profile_id:-<unknown>} (${profile})"
  "$ROOT/scripts/openai-oauth.sh" check --base-url "$base_url" || true
done

if [[ "$oauth_profiles" -eq 0 ]]; then
  echo "OpenAI OAuth profiles: none enabled"
fi
