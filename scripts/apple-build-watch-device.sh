#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"
SCHEME="${APPLE_WATCH_SCHEME:-VelWatch}"
CONFIGURATION="${APPLE_BUILD_CONFIGURATION:-Debug}"
DERIVED_DATA_PATH="${APPLE_WATCH_DERIVED_DATA_PATH:-$ROOT/var/apple-device-build/watch}"
TEAM_ID="${APPLE_DEVELOPMENT_TEAM:-}"
IOS_BUNDLE_ID="${APPLE_IOS_BUNDLE_ID:-vel.VeliOS}"
BUNDLE_ID="${APPLE_WATCH_BUNDLE_ID:-${IOS_BUNDLE_ID}.watchkitapp}"
COMPANION_BUNDLE_ID="${APPLE_WATCH_COMPANION_BUNDLE_ID:-$IOS_BUNDLE_ID}"

if [[ ! -d "$PROJECT" ]]; then
  echo "apple-build-watch-device: missing Xcode project at $PROJECT" >&2
  exit 1
fi

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "apple-build-watch-device: xcodebuild is required" >&2
  exit 1
fi

if ! xcodebuild -showsdks | rg -q "watchos"; then
  echo "apple-build-watch-device: watchOS SDK not installed" >&2
  exit 1
fi

if ! xcodebuild -list -project "$PROJECT" | rg -q "^[[:space:]]+$SCHEME$"; then
  echo "apple-build-watch-device: scheme '$SCHEME' not found in $PROJECT" >&2
  exit 1
fi

DESTINATIONS="$(xcodebuild -project "$PROJECT" -scheme "$SCHEME" -showdestinations 2>&1 || true)"
if printf '%s\n' "$DESTINATIONS" | rg -q "watchOS .* is not installed"; then
  echo "apple-build-watch-device: watchOS platform/runtime is missing; install from Xcode > Settings > Components" >&2
  exit 1
fi

mkdir -p "$DERIVED_DATA_PATH"

if [[ -z "$TEAM_ID" ]]; then
  echo "apple-build-watch-device: APPLE_DEVELOPMENT_TEAM is not set; using project default signing team" >&2
fi

echo "apple-build-watch-device: building scheme '$SCHEME' for generic watchOS device"
echo "apple-build-watch-device: watch bundle '$BUNDLE_ID' (companion: '$COMPANION_BUNDLE_ID')"

BUILD_ARGS=(
  -project "$PROJECT"
  -scheme "$SCHEME"
  -configuration "$CONFIGURATION"
  -destination "generic/platform=watchOS"
  -derivedDataPath "$DERIVED_DATA_PATH"
  -allowProvisioningUpdates
)

if [[ -n "$TEAM_ID" ]]; then
  BUILD_ARGS+=(DEVELOPMENT_TEAM="$TEAM_ID")
fi

BUILD_ARGS+=(PRODUCT_BUNDLE_IDENTIFIER="$BUNDLE_ID")
BUILD_ARGS+=(INFOPLIST_KEY_WKCompanionAppBundleIdentifier="$COMPANION_BUNDLE_ID")

xcodebuild "${BUILD_ARGS[@]}" build

APP_DIR="$DERIVED_DATA_PATH/Build/Products/${CONFIGURATION}-watchos"
APP_PATH="$(find "$APP_DIR" -maxdepth 1 -type d -name '*.app' | head -n1)"

if [[ -z "$APP_PATH" || ! -d "$APP_PATH" ]]; then
  echo "apple-build-watch-device: build succeeded but .app bundle not found under $APP_DIR" >&2
  exit 1
fi

mkdir -p "$ROOT/var/apple-device-build"
printf '%s\n' "$APP_PATH" > "$ROOT/var/apple-device-build/last-watch-app-path.txt"

echo "apple-build-watch-device: built app at $APP_PATH"
