#!/usr/bin/env bash
# Start veld only, but fail early if the active chat/fallback profile expects a localhost
# OpenAI OAuth proxy that is missing.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

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

ensure_openai_oauth_ready() {
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
  local profile_id profile_file provider enabled base_url
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
      echo "OpenAI OAuth profile $profile_id is missing base_url in $profile_file" >&2
      exit 1
    fi

    echo "Checking OpenAI OAuth proxy for active profile $profile_id..."
    if ! "$ROOT/scripts/openai-oauth.sh" check --base-url "$base_url"; then
      :
    fi
    if ! curl -fsS --max-time 3 "${base_url%/}/models" >/dev/null 2>&1; then
      echo "Active chat routing expects OpenAI OAuth at $base_url, but it is not reachable." >&2
      echo "Run: $ROOT/scripts/openai-oauth.sh run --base-url $base_url" >&2
      exit 1
    fi
  done
}

ensure_openai_oauth_ready
exec cargo run -p veld
