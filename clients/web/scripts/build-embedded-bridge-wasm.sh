#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
CRATE_DIR="$ROOT_DIR/crates/vel-embedded-bridge"
OUT_DIR="$ROOT_DIR/clients/web/public/embedded-bridge"
RUST_TOOLCHAIN_FILE="$ROOT_DIR/rust-toolchain.toml"

mkdir -p "$OUT_DIR"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "wasm-pack is required to build the embedded bridge browser artifact." >&2
  echo "Install it with: cargo install wasm-pack" >&2
  exit 1
fi

configure_rust_toolchain() {
  if [[ -d "$HOME/.cargo/bin" ]]; then
    export PATH="$HOME/.cargo/bin:$PATH"
  fi

  if ! command -v rustup >/dev/null 2>&1; then
    return 0
  fi

  if [[ -z "${RUSTUP_TOOLCHAIN:-}" && -f "$RUST_TOOLCHAIN_FILE" ]]; then
    local channel
    channel="$(sed -nE 's/^[[:space:]]*channel[[:space:]]*=[[:space:]]*"([^"]+)".*$/\1/p' "$RUST_TOOLCHAIN_FILE" | head -n1)"
    if [[ -n "$channel" ]]; then
      export RUSTUP_TOOLCHAIN="$channel"
    fi
  fi

  local toolchain_ref="${RUSTUP_TOOLCHAIN:-}"
  if [[ -z "$toolchain_ref" ]]; then
    toolchain_ref="$(rustup show active-toolchain 2>/dev/null | awk 'NR == 1 { print $1 }')"
  fi

  if [[ -n "$toolchain_ref" ]] \
    && ! rustup target list --toolchain "$toolchain_ref" --installed | grep -qx 'wasm32-unknown-unknown'; then
    echo "Installing wasm32-unknown-unknown for rustup toolchain $toolchain_ref..." >&2
    rustup target add wasm32-unknown-unknown --toolchain "$toolchain_ref"
  fi
}

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

configure_rust_toolchain

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
