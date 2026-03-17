#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"
SCHEME="${APPLE_IOS_SCHEME:-VeliOS}"
CONFIGURATION="${APPLE_BUILD_CONFIGURATION:-Debug}"
DERIVED_DATA_PATH="${APPLE_IOS_DERIVED_DATA_PATH:-$ROOT/var/apple-device-build/ios}"
TEAM_ID="${APPLE_DEVELOPMENT_TEAM:-}"
BUNDLE_ID="${APPLE_IOS_BUNDLE_ID:-vel.VeliOS}"

if [[ ! -d "$PROJECT" ]]; then
  echo "apple-build-ios-device: missing Xcode project at $PROJECT" >&2
  exit 1
fi

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "apple-build-ios-device: xcodebuild is required" >&2
  exit 1
fi

if ! xcodebuild -showsdks | rg -q "iphoneos"; then
  echo "apple-build-ios-device: iOS device SDK not installed" >&2
  exit 1
fi

mkdir -p "$DERIVED_DATA_PATH"

if [[ -z "$TEAM_ID" ]]; then
  echo "apple-build-ios-device: APPLE_DEVELOPMENT_TEAM is not set; using project default signing team" >&2
fi

echo "apple-build-ios-device: building scheme '$SCHEME' for generic iOS device"

BUILD_ARGS=(
  -project "$PROJECT"
  -scheme "$SCHEME"
  -configuration "$CONFIGURATION"
  -destination "generic/platform=iOS"
  -derivedDataPath "$DERIVED_DATA_PATH"
  -allowProvisioningUpdates
)

if [[ -n "$TEAM_ID" ]]; then
  BUILD_ARGS+=(DEVELOPMENT_TEAM="$TEAM_ID")
fi

BUILD_ARGS+=(PRODUCT_BUNDLE_IDENTIFIER="$BUNDLE_ID")

xcodebuild "${BUILD_ARGS[@]}" build

APP_DIR="$DERIVED_DATA_PATH/Build/Products/${CONFIGURATION}-iphoneos"
APP_PATH="$(find "$APP_DIR" -maxdepth 1 -type d -name '*.app' | head -n1)"

if [[ -z "$APP_PATH" || ! -d "$APP_PATH" ]]; then
  echo "apple-build-ios-device: build succeeded but .app bundle not found under $APP_DIR" >&2
  exit 1
fi

mkdir -p "$ROOT/var/apple-device-build"
printf '%s\n' "$APP_PATH" > "$ROOT/var/apple-device-build/last-ios-app-path.txt"

echo "apple-build-ios-device: built app at $APP_PATH"
