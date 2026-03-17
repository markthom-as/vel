#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"
SCHEME="${APPLE_IOS_SCHEME:-VeliOS}"

if [[ ! -d "$PROJECT" ]]; then
  echo "apple-build-ios-sim: missing Xcode project at $PROJECT" >&2
  exit 1
fi

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "apple-build-ios-sim: xcodebuild is required" >&2
  exit 1
fi

if ! xcodebuild -showsdks | rg -q "iphoneos|iphonesimulator"; then
  echo "apple-build-ios-sim: missing iOS SDK; run 'make apple-setup-simulator'" >&2
  exit 1
fi

DEVICE_ID="${APPLE_SIM_DEVICE_ID:-}"

if [[ -z "$DEVICE_ID" ]]; then
  DEVICE_ID="$(xcrun simctl list devices available \
    | rg -m1 '^[[:space:]]+iPhone.*\(([0-9A-F-]{36})\)' -or '$1')"
fi

if [[ -z "$DEVICE_ID" ]]; then
  echo "apple-build-ios-sim: no available iPhone simulator found; run 'make apple-setup-simulator'" >&2
  exit 1
fi

echo "apple-build-ios-sim: building scheme '$SCHEME' for simulator device $DEVICE_ID"
xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -destination "id=$DEVICE_ID" \
  build
