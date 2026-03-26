#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"
SCHEME="${APPLE_IOS_SCHEME:-VeliOS}"
CONFIGURATION="${APPLE_BUILD_CONFIGURATION:-Debug}"

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
  DEVICE_ID="$(xcrun simctl list devices available | awk '
    /^-- iOS / {
      version=$3
      gsub(/[^0-9.]/, "", version)
      split(version, parts, ".")
      major=(parts[1] == "" ? 0 : parts[1] + 0)
      minor=(parts[2] == "" ? 0 : parts[2] + 0)
      patch=(parts[3] == "" ? 0 : parts[3] + 0)
      currentKey=sprintf("%04d%04d%04d", major, minor, patch)
      next
    }
    /^[[:space:]]+iPhone/ {
      if (match($0, /\(([0-9A-F-]{36})\)/)) {
        id=substr($0, RSTART + 1, RLENGTH - 2)
        if (currentKey > bestKey) {
          bestKey=currentKey
          bestId=id
        }
      }
    }
    END {
      print bestId
    }
  ')"
fi

if [[ -z "$DEVICE_ID" ]]; then
  echo "apple-build-ios-sim: no available iPhone simulator found; run 'make apple-setup-simulator'" >&2
  exit 1
fi

echo "apple-build-ios-sim: building scheme '$SCHEME' for simulator device $DEVICE_ID"
xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -configuration "$CONFIGURATION" \
  -destination "id=$DEVICE_ID" \
  build

BUILD_SETTINGS="$(xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -destination "id=$DEVICE_ID" \
  -configuration "$CONFIGURATION" \
  -showBuildSettings)"

TARGET_BUILD_DIR="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*TARGET_BUILD_DIR = //p' | tail -n1)"
WRAPPER_NAME="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*WRAPPER_NAME = //p' | tail -n1)"
APP_PATH="$TARGET_BUILD_DIR/$WRAPPER_NAME"

if [[ -z "$TARGET_BUILD_DIR" || -z "$WRAPPER_NAME" || ! -d "$APP_PATH" ]]; then
  echo "apple-build-ios-sim: unable to resolve built app path after bridge copy step." >&2
  exit 1
fi

APPLE_EMBEDDED_BRIDGE_PLATFORM=simulator \
APPLE_EMBEDDED_BRIDGE_PROFILE="$CONFIGURATION" \
APPLE_EMBEDDED_BRIDGE_APP_PATH="$APP_PATH" \
bash "$ROOT/scripts/apple-package-embedded-bridge.sh"
