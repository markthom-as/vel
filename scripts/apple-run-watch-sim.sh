#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"
SCHEME="${APPLE_WATCH_SCHEME:-VelWatch}"

if [[ ! -d "$PROJECT" ]]; then
  echo "apple-run-watch-sim: missing Xcode project at $PROJECT" >&2
  exit 1
fi

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "apple-run-watch-sim: xcodebuild is required" >&2
  exit 1
fi

if ! xcodebuild -showsdks | rg -q "watchos|watchsimulator"; then
  echo "apple-run-watch-sim: missing watchOS SDK; run 'make apple-setup-simulator'" >&2
  exit 1
fi

WATCH_DEVICE_ID="${APPLE_WATCH_SIM_DEVICE_ID:-}"
PHONE_DEVICE_ID="${APPLE_SIM_DEVICE_ID:-}"

if [[ -z "$WATCH_DEVICE_ID" ]]; then
  WATCH_DEVICE_ID="$(xcrun simctl list devices available | awk '
    /^-- watchOS / {
      version=$3
      gsub(/[^0-9.]/, "", version)
      split(version, parts, ".")
      major=(parts[1] == "" ? 0 : parts[1] + 0)
      minor=(parts[2] == "" ? 0 : parts[2] + 0)
      patch=(parts[3] == "" ? 0 : parts[3] + 0)
      currentKey=sprintf("%04d%04d%04d", major, minor, patch)
      next
    }
    /^[[:space:]]+Apple Watch/ {
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

if [[ -z "$PHONE_DEVICE_ID" ]]; then
  PHONE_DEVICE_ID="$(xcrun simctl list devices available | awk '
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

if [[ -z "$WATCH_DEVICE_ID" ]]; then
  echo "apple-run-watch-sim: no available Apple Watch simulator found; run 'make apple-setup-simulator'" >&2
  exit 1
fi

if [[ -z "$PHONE_DEVICE_ID" ]]; then
  echo "apple-run-watch-sim: no available iPhone simulator found; run 'make apple-setup-simulator'" >&2
  exit 1
fi

echo "apple-run-watch-sim: building scheme '$SCHEME' for watch simulator $WATCH_DEVICE_ID"
xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -destination "id=$WATCH_DEVICE_ID" \
  build >/dev/null

BUILD_SETTINGS="$(xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -destination "id=$WATCH_DEVICE_ID" \
  -showBuildSettings)"

TARGET_BUILD_DIR="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*TARGET_BUILD_DIR = //p' | tail -n1)"
WRAPPER_NAME="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*WRAPPER_NAME = //p' | tail -n1)"
BUNDLE_ID="$(printf '%s\n' "$BUILD_SETTINGS" | sed -n 's/^[[:space:]]*PRODUCT_BUNDLE_IDENTIFIER = //p' | tail -n1)"

if [[ -z "$TARGET_BUILD_DIR" || -z "$WRAPPER_NAME" || -z "$BUNDLE_ID" ]]; then
  echo "apple-run-watch-sim: unable to resolve watch app bundle path/build settings" >&2
  exit 1
fi

APP_PATH="$TARGET_BUILD_DIR/$WRAPPER_NAME"
if [[ ! -d "$APP_PATH" ]]; then
  echo "apple-run-watch-sim: watch app bundle not found at $APP_PATH" >&2
  exit 1
fi

open -a Simulator
xcrun simctl boot "$PHONE_DEVICE_ID" >/dev/null 2>&1 || true
xcrun simctl boot "$WATCH_DEVICE_ID" >/dev/null 2>&1 || true
xcrun simctl install "$WATCH_DEVICE_ID" "$APP_PATH"
PID="$(xcrun simctl launch "$WATCH_DEVICE_ID" "$BUNDLE_ID" | awk -F': ' '{print $2}' | tail -n1)"
echo "apple-run-watch-sim: launched $BUNDLE_ID on watch (pid: ${PID:-unknown})"
