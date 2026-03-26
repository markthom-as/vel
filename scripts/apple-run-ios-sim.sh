#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"
SCHEME="${APPLE_IOS_SCHEME:-VeliOS}"
CONFIGURATION="${APPLE_BUILD_CONFIGURATION:-Debug}"

if [[ ! -d "$PROJECT" ]]; then
  echo "apple-run-ios-sim: missing Xcode project at $PROJECT" >&2
  exit 1
fi

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "apple-run-ios-sim: xcodebuild is required" >&2
  exit 1
fi

if ! xcodebuild -showsdks | rg -q "iphoneos|iphonesimulator"; then
  echo "apple-run-ios-sim: missing iOS SDK; run 'make apple-setup-simulator'" >&2
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
  echo "apple-run-ios-sim: no available iPhone simulator found; run 'make apple-setup-simulator'" >&2
  exit 1
fi

echo "apple-run-ios-sim: building scheme '$SCHEME' for simulator device $DEVICE_ID"
xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -configuration "$CONFIGURATION" \
  -destination "id=$DEVICE_ID" \
  build >/dev/null

BUILD_SETTINGS="$(xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -configuration "$CONFIGURATION" \
  -destination "id=$DEVICE_ID" \
  -showBuildSettings)"

TARGET_BUILD_DIR="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*TARGET_BUILD_DIR = //p' | tail -n1)"
WRAPPER_NAME="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*WRAPPER_NAME = //p' | tail -n1)"
BUNDLE_ID="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*PRODUCT_BUNDLE_IDENTIFIER = //p' | tail -n1)"

if [[ -z "$TARGET_BUILD_DIR" || -z "$WRAPPER_NAME" || -z "$BUNDLE_ID" ]]; then
  echo "apple-run-ios-sim: unable to resolve app bundle path/build settings" >&2
  exit 1
fi

APP_PATH="$TARGET_BUILD_DIR/$WRAPPER_NAME"
if [[ ! -d "$APP_PATH" ]]; then
  echo "apple-run-ios-sim: app bundle not found at $APP_PATH" >&2
  exit 1
fi

APPLE_EMBEDDED_BRIDGE_PLATFORM=simulator \
APPLE_EMBEDDED_BRIDGE_PROFILE="$CONFIGURATION" \
APPLE_EMBEDDED_BRIDGE_APP_PATH="$APP_PATH" \
bash "$ROOT/scripts/apple-package-embedded-bridge.sh"

open -a Simulator
xcrun simctl boot "$DEVICE_ID" >/dev/null 2>&1 || true
xcrun simctl install "$DEVICE_ID" "$APP_PATH"
PID="$(xcrun simctl launch "$DEVICE_ID" "$BUNDLE_ID" | awk -F': ' '{print $2}' | tail -n1)"
echo "apple-run-ios-sim: launched $BUNDLE_ID (pid: ${PID:-unknown})"
