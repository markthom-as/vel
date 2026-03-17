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

if ! xcrun simctl list devices available | rg -q "iPhone"; then
  echo "apple-setup-simulator: no available iPhone simulator runtimes found" >&2
  echo "Install an iOS Simulator runtime from Xcode > Settings > Components." >&2
  exit 1
fi

echo "apple-setup-simulator: iOS simulator runtime is available"
