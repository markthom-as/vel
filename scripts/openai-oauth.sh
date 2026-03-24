#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

usage() {
  cat <<'EOF'
Usage:
  scripts/openai-oauth.sh check [--base-url URL]
  scripts/openai-oauth.sh run [--base-url URL]

Environment:
  OPENAI_OAUTH_BASE_URL      Default base URL when --base-url is omitted.
  OPENAI_OAUTH_PACKAGE       npm package to execute. Default: openai-oauth@latest
  OPENAI_OAUTH_AUTH_FILE     Override auth.json path.
  OPENAI_OAUTH_MODELS        Optional comma-separated model allowlist.
  OPENAI_OAUTH_CODEX_VERSION Optional Codex API version override.
  OPENAI_OAUTH_UPSTREAM_URL  Optional upstream Codex base URL override.
  OPENAI_OAUTH_CLIENT_ID     Optional OAuth client id override.
  OPENAI_OAUTH_TOKEN_URL     Optional OAuth token URL override.
EOF
}

find_auth_file() {
  local candidates=()
  if [[ -n "${OPENAI_OAUTH_AUTH_FILE:-}" ]]; then
    candidates+=("${OPENAI_OAUTH_AUTH_FILE}")
  fi
  if [[ -n "${CHATGPT_LOCAL_HOME:-}" ]]; then
    candidates+=("${CHATGPT_LOCAL_HOME}/auth.json")
  fi
  if [[ -n "${CODEX_HOME:-}" ]]; then
    candidates+=("${CODEX_HOME}/auth.json")
  fi
  candidates+=(
    "$HOME/.chatgpt-local/auth.json"
    "$HOME/.codex/auth.json"
  )

  local candidate
  for candidate in "${candidates[@]}"; do
    if [[ -f "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

parse_base_url() {
  local url="$1"
  if [[ ! "$url" =~ ^https?:// ]]; then
    echo "openai-oauth base URL must use http or https: $url" >&2
    exit 1
  fi

  local without_scheme="${url#*://}"
  local host_port="${without_scheme%%/*}"
  OAUTH_HOST="${host_port%%:*}"
  OAUTH_PORT="${host_port##*:}"
  OAUTH_PATH="/${without_scheme#*/}"

  if [[ -z "$OAUTH_HOST" || -z "$OAUTH_PORT" || "$OAUTH_HOST" == "$host_port" ]]; then
    echo "openai-oauth base URL must include an explicit host and port: $url" >&2
    exit 1
  fi
}

healthcheck() {
  curl -fsS --max-time 5 "${BASE_URL%/}/models" >/dev/null 2>&1
}

COMMAND="${1:-run}"
if [[ $# -gt 0 ]]; then
  shift
fi
BASE_URL="${OPENAI_OAUTH_BASE_URL:-http://127.0.0.1:8014/v1}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-url)
      BASE_URL="${2:-}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

parse_base_url "$BASE_URL"

PACKAGE="${OPENAI_OAUTH_PACKAGE:-openai-oauth@latest}"
AUTH_FILE="$(find_auth_file || true)"

case "$COMMAND" in
  check)
    echo "OpenAI OAuth base URL: $BASE_URL"
    if command -v npx >/dev/null 2>&1; then
      echo "npx: $(command -v npx)"
    else
      echo "npx: not found"
    fi

    if [[ -n "$AUTH_FILE" ]]; then
      echo "auth file: found at $AUTH_FILE"
    else
      echo "auth file: missing"
      echo "login: run npx @openai/codex login"
    fi

    if healthcheck; then
      echo "proxy health: ready"
    else
      echo "proxy health: not reachable at ${BASE_URL%/}/models"
    fi
    ;;
  run)
    if healthcheck; then
      echo "OpenAI OAuth proxy already ready at $BASE_URL"
      exit 0
    fi
    if ! command -v npx >/dev/null 2>&1; then
      echo "OpenAI OAuth proxy requires npx. Install Node.js/npm first." >&2
      exit 1
    fi
    if [[ -z "$AUTH_FILE" ]]; then
      echo "OpenAI OAuth auth file not found." >&2
      echo "Run: npx @openai/codex login" >&2
      exit 1
    fi

    args=(--yes "$PACKAGE" --host "$OAUTH_HOST" --port "$OAUTH_PORT" --oauth-file "$AUTH_FILE")
    if [[ -n "${OPENAI_OAUTH_MODELS:-}" ]]; then
      args+=(--models "$OPENAI_OAUTH_MODELS")
    fi
    if [[ -n "${OPENAI_OAUTH_CODEX_VERSION:-}" ]]; then
      args+=(--codex-version "$OPENAI_OAUTH_CODEX_VERSION")
    fi
    if [[ -n "${OPENAI_OAUTH_UPSTREAM_URL:-}" ]]; then
      args+=(--base-url "$OPENAI_OAUTH_UPSTREAM_URL")
    fi
    if [[ -n "${OPENAI_OAUTH_CLIENT_ID:-}" ]]; then
      args+=(--oauth-client-id "$OPENAI_OAUTH_CLIENT_ID")
    fi
    if [[ -n "${OPENAI_OAUTH_TOKEN_URL:-}" ]]; then
      args+=(--oauth-token-url "$OPENAI_OAUTH_TOKEN_URL")
    fi

    echo "Starting OpenAI OAuth proxy at $BASE_URL"
    echo "Using auth file: $AUTH_FILE"
    exec npx "${args[@]}"
    ;;
  *)
    echo "unknown command: $COMMAND" >&2
    usage >&2
    exit 2
    ;;
esac
