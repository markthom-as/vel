#!/usr/bin/env bash
set -euo pipefail

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "apple-setup-simulator: xcodebuild is required" >&2
  exit 1
fi

echo "apple-setup-simulator: running xcodebuild -runFirstLaunch"
xcodebuild -runFirstLaunch

if ! xcodebuild -showsdks | rg -q "iphoneos|iphonesimulator"; then
  echo "apple-setup-simulator: iOS SDK not installed; downloading platform"
  xcodebuild -downloadPlatform iOS
fi

if ! xcodebuild -showsdks | rg -q "watchos|watchsimulator"; then
  echo "apple-setup-simulator: watchOS SDK not installed; downloading platform"
  xcodebuild -downloadPlatform watchOS
fi

if ! xcrun simctl list devices available | rg -q "iPhone"; then
  echo "apple-setup-simulator: no available iPhone simulator runtimes found" >&2
  echo "Install an iOS Simulator runtime from Xcode > Settings > Components." >&2
  exit 1
fi

if ! xcrun simctl list devices available | rg -q "Apple Watch"; then
  echo "apple-setup-simulator: no available Apple Watch simulator runtimes found" >&2
  echo "Install a watchOS Simulator runtime from Xcode > Settings > Components." >&2
  exit 1
fi

IPHONE_ID="$(xcrun simctl list devices available | awk '
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
  END { print bestId }
')"

WATCH_ID="$(xcrun simctl list devices available | awk '
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
  END { print bestId }
')"

if [[ -n "$IPHONE_ID" ]]; then
  xcrun simctl boot "$IPHONE_ID" >/dev/null 2>&1 || true
fi
if [[ -n "$WATCH_ID" ]]; then
  xcrun simctl boot "$WATCH_ID" >/dev/null 2>&1 || true
fi

echo "apple-setup-simulator: iOS and watchOS simulator runtimes are available"
echo "apple-setup-simulator: newest iPhone simulator $IPHONE_ID"
echo "apple-setup-simulator: newest Watch simulator $WATCH_ID"
