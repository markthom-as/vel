#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IOS_DEVICE_ID="${APPLE_DEVICE_ID:-}"
WATCH_DEVICE_ID="${APPLE_WATCH_DEVICE_ID:-}"
IOS_BUNDLE_ID="${APPLE_IOS_BUNDLE_ID:-vel.VeliOS}"
WATCH_BUNDLE_ID="${APPLE_WATCH_BUNDLE_ID:-${IOS_BUNDLE_ID}.watchkitapp}"
WATCH_COMPANION_BUNDLE_ID="${APPLE_WATCH_COMPANION_BUNDLE_ID:-$IOS_BUNDLE_ID}"

if ! command -v xcrun >/dev/null 2>&1; then
  echo "apple-install-devices: xcrun is required" >&2
  exit 1
fi

if [[ -z "$IOS_DEVICE_ID" ]]; then
  echo "apple-install-devices: APPLE_DEVICE_ID is required (iPhone or iPad UDID)." >&2
  echo "Run 'make apple-list-devices' to discover connected device IDs." >&2
  exit 1
fi

if [[ -z "${APPLE_DEVELOPMENT_TEAM:-}" ]]; then
  echo "apple-install-devices: warning: APPLE_DEVELOPMENT_TEAM is not set; project default team will be used."
fi

echo "apple-install-devices: installing iOS app bundle '$IOS_BUNDLE_ID' to device '$IOS_DEVICE_ID'"
"$ROOT/scripts/apple-build-ios-device.sh"
"$ROOT/scripts/apple-install-ios-device.sh"

if [[ -n "$WATCH_DEVICE_ID" ]]; then
  export APPLE_WATCH_BUNDLE_ID="$WATCH_BUNDLE_ID"
  export APPLE_WATCH_COMPANION_BUNDLE_ID="$WATCH_COMPANION_BUNDLE_ID"

  echo "apple-install-devices: installing watch app bundle '$WATCH_BUNDLE_ID' to watch '$WATCH_DEVICE_ID'"
  echo "apple-install-devices: watch companion bundle is '$WATCH_COMPANION_BUNDLE_ID'"
  "$ROOT/scripts/apple-build-watch-device.sh"
  "$ROOT/scripts/apple-install-watch-device.sh"
else
  echo "apple-install-devices: APPLE_WATCH_DEVICE_ID is not set; skipping watch install."
fi

echo "apple-install-devices: complete"
