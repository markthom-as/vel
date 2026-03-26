#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
APP_PATH="${APPLE_EMBEDDED_BRIDGE_APP_PATH:-}"
PLATFORM="${APPLE_EMBEDDED_BRIDGE_PLATFORM:-auto}"
BUILD_PROFILE="${APPLE_EMBEDDED_BRIDGE_PROFILE:-Debug}"
CARGO_PROFILE="$(printf '%s' "$BUILD_PROFILE" | tr '[:upper:]' '[:lower:]')"
LIB_NAME="libvel_embedded_bridge.dylib"
ENABLE="${APPLE_EMBEDDED_BRIDGE_ENABLE:-1}"

if [[ -z "$APP_PATH" || ! -d "$APP_PATH" ]]; then
  echo "apple-package-embedded-bridge: missing app bundle path (APPLE_EMBEDDED_BRIDGE_APP_PATH)." >&2
  exit 1
fi

if [[ "$ENABLE" == "0" ]]; then
  echo "apple-package-embedded-bridge: disabled via APPLE_EMBEDDED_BRIDGE_ENABLE=0."
  exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "apple-package-embedded-bridge: cargo not available; install Rust toolchain and rerun." >&2
  exit 1
fi

if ! command -v rustup >/dev/null 2>&1; then
  echo "apple-package-embedded-bridge: rustup not available; skipping bridge embed." >&2
  exit 1
fi

case "$CARGO_PROFILE" in
  debug | release)
    ;;
  *)
    echo "apple-package-embedded-bridge: unknown profile '$BUILD_PROFILE', defaulting to debug."
    CARGO_PROFILE="debug"
    ;;
esac

if [[ "$PLATFORM" == "simulator" || "$PLATFORM" == "auto" ]]; then
  if [[ "$(uname -m)" == "arm64" ]]; then
    TARGET="aarch64-apple-ios-sim"
  else
    TARGET="x86_64-apple-ios"
  fi
elif [[ "$PLATFORM" == "device" ]]; then
  TARGET="aarch64-apple-ios"
else
  echo "apple-package-embedded-bridge: unsupported platform '$PLATFORM'." >&2
  exit 1
fi

if ! rustup target list --installed | rg -q "^${TARGET}$"; then
  echo "apple-package-embedded-bridge: adding rust target $TARGET"
  rustup target add "$TARGET"
fi

echo "apple-package-embedded-bridge: building vel-embedded-bridge ($CARGO_PROFILE) for $TARGET"
cargo build \
  --manifest-path "$ROOT/Cargo.toml" \
  -p vel-embedded-bridge \
  --target "$TARGET" \
  --"$CARGO_PROFILE"

BRIDGE_SRC="$ROOT/target/$TARGET/$CARGO_PROFILE/$LIB_NAME"
if [[ ! -f "$BRIDGE_SRC" ]]; then
  echo "apple-package-embedded-bridge: expected bridge artifact missing at $BRIDGE_SRC" >&2
  exit 1
fi

BRIDGE_DST_DIR="$APP_PATH/Frameworks"
mkdir -p "$BRIDGE_DST_DIR"
cp "$BRIDGE_SRC" "$BRIDGE_DST_DIR/$LIB_NAME"
chmod 755 "$BRIDGE_DST_DIR/$LIB_NAME"

echo "apple-package-embedded-bridge: copied bridge to $BRIDGE_DST_DIR/$LIB_NAME"
