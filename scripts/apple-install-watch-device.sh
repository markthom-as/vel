#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LAST_APP_PATH_FILE="$ROOT/var/apple-device-build/last-watch-app-path.txt"
DEVICE_ID="${APPLE_WATCH_DEVICE_ID:-}"
APP_PATH="${APPLE_WATCH_APP_PATH:-}"

if ! command -v xcrun >/dev/null 2>&1; then
  echo "apple-install-watch-device: xcrun is required" >&2
  exit 1
fi

if [[ -z "$DEVICE_ID" ]]; then
  echo "apple-install-watch-device: set APPLE_WATCH_DEVICE_ID to your Apple Watch UDID (see: xcrun devicectl list devices)" >&2
  exit 1
fi

if [[ -z "$APP_PATH" && -f "$LAST_APP_PATH_FILE" ]]; then
  APP_PATH="$(cat "$LAST_APP_PATH_FILE")"
fi

if [[ -z "$APP_PATH" || ! -d "$APP_PATH" ]]; then
  "$ROOT/scripts/apple-build-watch-device.sh"
  APP_PATH="$(cat "$LAST_APP_PATH_FILE")"
fi

if [[ ! -d "$APP_PATH" ]]; then
  echo "apple-install-watch-device: app bundle not found at $APP_PATH" >&2
  exit 1
fi

echo "apple-install-watch-device: installing $APP_PATH on watch $DEVICE_ID"
xcrun devicectl device install app --device "$DEVICE_ID" "$APP_PATH"

BUNDLE_ID="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$APP_PATH/Info.plist" 2>/dev/null || true)"
if [[ -n "$BUNDLE_ID" ]]; then
  echo "apple-install-watch-device: launching $BUNDLE_ID"
  xcrun devicectl device process launch --device "$DEVICE_ID" --terminate-existing "$BUNDLE_ID" >/dev/null || true
fi

echo "apple-install-watch-device: complete"
