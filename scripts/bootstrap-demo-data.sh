#!/usr/bin/env bash
# Populate a running local DB/API with starter data for demos.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

BASE_URL="${VEL_API_BASE:-http://127.0.0.1:4130}"
CLI_BASE_ARGS=()
if [[ "$BASE_URL" != "http://127.0.0.1:4130" ]]; then
  CLI_BASE_ARGS=("--base-url" "$BASE_URL")
fi

if ! curl -sf "$BASE_URL/v1/health" >/dev/null; then
  echo "bootstrap-demo-data requires a running veld API at $BASE_URL"
  exit 1
fi

echo "Bootstrapping demo captures and commitments..."
cargo run -p vel-cli "${CLI_BASE_ARGS[@]}" -- capture "Review project backlog for tomorrow." --type todo --source demo
cargo run -p vel-cli "${CLI_BASE_ARGS[@]}" -- capture "Call with operations to confirm schedule." --type note --source demo

echo "Creating sample commitments..."
cargo run -p vel-cli "${CLI_BASE_ARGS[@]}" -- commitment add "Review weekly synthesis outputs"
cargo run -p vel-cli "${CLI_BASE_ARGS[@]}" -- commitment add "Send status update to partner" --kind todo --project demo

if command -v cargo >/dev/null; then
  echo "Refreshing context to ensure deterministic run output..."
  cargo run -p vel-cli "${CLI_BASE_ARGS[@]}" -- today --json >/dev/null
fi

echo "Demo bootstrap complete."
