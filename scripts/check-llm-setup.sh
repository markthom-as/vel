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
