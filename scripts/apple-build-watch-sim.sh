#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"
SCHEME="${APPLE_WATCH_SCHEME:-VelWatch}"

if [[ ! -d "$PROJECT" ]]; then
  echo "apple-build-watch-sim: missing Xcode project at $PROJECT" >&2
  exit 1
fi

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "apple-build-watch-sim: xcodebuild is required" >&2
  exit 1
fi

if ! xcodebuild -showsdks | rg -q "watchos|watchsimulator"; then
  echo "apple-build-watch-sim: missing watchOS SDK; run 'make apple-setup-simulator'" >&2
  exit 1
fi

DEVICE_ID="${APPLE_WATCH_SIM_DEVICE_ID:-}"

if [[ -z "$DEVICE_ID" ]]; then
  DEVICE_ID="$(xcrun simctl list devices available | awk '
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

if [[ -z "$DEVICE_ID" ]]; then
  echo "apple-build-watch-sim: no available Apple Watch simulator found; run 'make apple-setup-simulator'" >&2
  exit 1
fi

echo "apple-build-watch-sim: building scheme '$SCHEME' for simulator device $DEVICE_ID"
xcodebuild \
  -project "$PROJECT" \
  -scheme "$SCHEME" \
  -destination "id=$DEVICE_ID" \
  build
