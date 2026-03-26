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

build_wasm() {
  wasm-pack build \
    "$CRATE_DIR" \
    --target web \
    --out-dir "$OUT_DIR" \
    --out-name vel-embedded-bridge \
    --no-pack \
    -- \
    --features browser-wasm
}

if command -v lld >/dev/null 2>&1; then
  build_wasm
  exit 0
fi

if ! command -v nix-shell >/dev/null 2>&1; then
  echo "lld is required to link the wasm artifact, and nix-shell is unavailable to supply it." >&2
  exit 1
fi

echo "lld not found; rebuilding inside nix-shell -p lld." >&2
nix-shell -p lld --run "
  export PATH=\$PATH
  $(printf '%q ' wasm-pack build "$CRATE_DIR" --target web --out-dir "$OUT_DIR" --out-name vel-embedded-bridge --no-pack -- --features browser-wasm)
"
