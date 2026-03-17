#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v swift >/dev/null 2>&1; then
  echo "check-apple-swift: swift is required on this host" >&2
  exit 1
fi

if command -v nix-shell >/dev/null 2>&1; then
  nix-shell --run 'cd clients/apple/VelAPI && swift --version && swift build'
else
  echo "check-apple-swift: nix-shell not found; using host swift toolchain"
  cd clients/apple/VelAPI
  swift --version
  swift build
fi
