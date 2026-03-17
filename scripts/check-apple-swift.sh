#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v nix-shell >/dev/null 2>&1; then
  echo "check-apple-swift: nix-shell is required on this host" >&2
  exit 1
fi

nix-shell --run 'cd clients/apple/VelAPI && swift --version && swift build'
