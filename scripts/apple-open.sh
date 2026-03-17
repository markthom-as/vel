#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/clients/apple/Vel.xcodeproj"

if [[ ! -d "$PROJECT" ]]; then
  echo "apple-open: missing Xcode project at $PROJECT" >&2
  exit 1
fi

open "$PROJECT"
