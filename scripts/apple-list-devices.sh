#!/usr/bin/env bash
set -euo pipefail

if ! command -v xcrun >/dev/null 2>&1; then
  echo "apple-list-devices: xcrun is required" >&2
  exit 1
fi

echo "apple-list-devices: connected physical Apple devices"
RAW_OUTPUT="$(xcrun devicectl list devices 2>&1 || true)"
printf '%s\n' "$RAW_OUTPUT"

if printf '%s\n' "$RAW_OUTPUT" | rg -q "^No devices found\\.$"; then
  cat <<'EOF'

No devices are currently visible to Xcode.
Before retrying:
1. connect device via USB (or trusted wireless debugging)
2. unlock device and tap "Trust This Computer"
3. in Xcode, confirm the device appears in Window > Devices and Simulators

Then run:
  make apple-list-devices
EOF
  exit 0
fi

cat <<'EOF'

Next step:
1. copy the iPhone/iPad identifier into APPLE_DEVICE_ID
2. copy Apple Watch identifier into APPLE_WATCH_DEVICE_ID (optional)
3. set your signing team and install

Example:
  export APPLE_DEVELOPMENT_TEAM=<YOUR_TEAM_ID>
  export APPLE_DEVICE_ID=<IPHONE_OR_IPAD_UDID>
  export APPLE_WATCH_DEVICE_ID=<WATCH_UDID>   # optional
  # optional per-team bundle IDs:
  export APPLE_IOS_BUNDLE_ID=vel.VeliOS.personal
  export APPLE_WATCH_BUNDLE_ID=vel.VeliOS.personal.watchkitapp
  make apple-install-devices
EOF
