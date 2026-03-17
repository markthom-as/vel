#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LAST_APP_PATH_FILE="$ROOT/var/apple-device-build/last-ios-app-path.txt"
DEVICE_ID="${APPLE_DEVICE_ID:-}"
APP_PATH="${APPLE_IOS_APP_PATH:-}"

if ! command -v xcrun >/dev/null 2>&1; then
  echo "apple-install-ios-device: xcrun is required" >&2
  exit 1
fi

if [[ -z "$DEVICE_ID" ]]; then
  echo "apple-install-ios-device: set APPLE_DEVICE_ID to your iPhone/iPad UDID (see: xcrun devicectl list devices)" >&2
  exit 1
fi

if [[ -z "$APP_PATH" && -f "$LAST_APP_PATH_FILE" ]]; then
  APP_PATH="$(cat "$LAST_APP_PATH_FILE")"
fi

if [[ -z "$APP_PATH" || ! -d "$APP_PATH" ]]; then
  "$ROOT/scripts/apple-build-ios-device.sh"
  APP_PATH="$(cat "$LAST_APP_PATH_FILE")"
fi

if [[ ! -d "$APP_PATH" ]]; then
  echo "apple-install-ios-device: app bundle not found at $APP_PATH" >&2
  exit 1
fi

echo "apple-install-ios-device: installing $APP_PATH on device $DEVICE_ID"
xcrun devicectl device install app --device "$DEVICE_ID" "$APP_PATH"

BUNDLE_ID="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$APP_PATH/Info.plist" 2>/dev/null || true)"
if [[ -n "$BUNDLE_ID" ]]; then
  echo "apple-install-ios-device: launching $BUNDLE_ID"
  xcrun devicectl device process launch --device "$DEVICE_ID" --terminate-existing "$BUNDLE_ID" >/dev/null || true
fi

echo "apple-install-ios-device: complete"
