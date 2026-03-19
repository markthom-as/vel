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

echo "apple-setup-simulator: iOS and watchOS simulator runtimes are available"
