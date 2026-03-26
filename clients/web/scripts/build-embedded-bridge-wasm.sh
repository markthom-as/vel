#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
CRATE_DIR="$ROOT_DIR/crates/vel-embedded-bridge"
OUT_DIR="$ROOT_DIR/clients/web/public/embedded-bridge"

mkdir -p "$OUT_DIR"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "wasm-pack is required to build the embedded bridge browser artifact." >&2
  echo "Install it with: cargo install wasm-pack" >&2
  exit 1
fi

wasm-pack build \
  "$CRATE_DIR" \
  --target web \
  --out-dir "$OUT_DIR" \
  --out-name vel-embedded-bridge \
  --no-pack \
  -- \
  --features browser-wasm
