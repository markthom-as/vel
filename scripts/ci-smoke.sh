#!/usr/bin/env bash
# End-to-end smoke test for daemon/API/CLI startup and minimal behavior.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

BASE_URL="${VEL_API_BASE:-http://127.0.0.1:4130}"
VELD_PID=
MANAGED_VELD=false

fail() {
  echo "[smoke] $*"
  exit 1
}

cleanup() {
  if [[ "$MANAGED_VELD" == "true" && -n "$VELD_PID" ]]; then
    if kill -0 "$VELD_PID" 2>/dev/null; then
      kill "$VELD_PID"
      wait "$VELD_PID" 2>/dev/null || true
    fi
  fi
}
trap cleanup EXIT INT TERM

if curl -sf "$BASE_URL/v1/health" >/dev/null; then
  echo "Using existing veld on $BASE_URL."
else
  echo "Starting veld for smoke test on default config."
  cargo run -p veld > /tmp/veld-smoke.log 2>&1 &
  VELD_PID=$!
  MANAGED_VELD=true

  for i in $(seq 1 60); do
    if curl -sf "$BASE_URL/v1/health" >/dev/null; then
      break
    fi
    if ! kill -0 "$VELD_PID" 2>/dev/null; then
      fail "veld exited before becoming healthy"
    fi
    sleep 1
  done

  if ! curl -sf "$BASE_URL/v1/health" >/dev/null; then
    fail "veld did not become healthy"
  fi
fi

echo "Verifying CLI health..."
cargo run -p vel-cli --base-url "$BASE_URL" -- health

SMOKE_TEXT="Smoke capture $(date +%s)"
CAPTURE_OUTPUT="$(cargo run -p vel-cli --base-url "$BASE_URL" -- capture "$SMOKE_TEXT" --type quick_note --source smoke 2>&1)"
CAPTURE_ID="$(printf '%s\n' "$CAPTURE_OUTPUT" | sed -n 's/^capture_id: //p' | tr -d '[:space:]')"
if [[ -z "$CAPTURE_ID" ]]; then
  fail "smoke capture did not return capture_id"
fi

echo "Verifying recent captures include $CAPTURE_ID..."
RECENT_JSON="$(cargo run -p vel-cli --base-url "$BASE_URL" -- recent --limit 5 --json 2>&1)"
printf '%s\n' "$RECENT_JSON" | grep -q "\"capture_id\": \"$CAPTURE_ID\"" || fail "capture missing from recent list"

echo "Running context endpoint..."
cargo run -p vel-cli --base-url "$BASE_URL" -- today --json >/dev/null

echo "Running search endpoint..."
SEARCH_JSON="$(cargo run -p vel-cli --base-url "$BASE_URL" -- search "$SMOKE_TEXT" --json 2>&1)"
printf '%s\n' "$SEARCH_JSON" | grep -q "\"capture_id\": \"$CAPTURE_ID\"" || fail "capture missing from search results"

echo "Smoke test complete."
