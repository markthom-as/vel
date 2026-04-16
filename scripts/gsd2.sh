#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

usage() {
  cat <<'EOF'
Usage: scripts/gsd2.sh [--cli] [gsd arguments...]

Runs the installed GSD 2 command surface with a Node >=22 runtime.

Environment:
  GSD_NODE_BIN   Absolute path to the node binary to use.
  GSD_BIN        GSD command to run. Defaults to gsd, or gsd-cli with --cli.
EOF
}

node_major() {
  "$1" -p 'Number(process.versions.node.split(".")[0])' 2>/dev/null || return 1
}

find_node22() {
  local candidates=()

  if [[ -n "${GSD_NODE_BIN:-}" ]]; then
    candidates+=("$GSD_NODE_BIN")
  fi

  candidates+=(
    "/opt/homebrew/opt/node@22/bin/node"
    "/opt/homebrew/opt/node/bin/node"
    "/usr/local/opt/node@22/bin/node"
    "/usr/local/opt/node/bin/node"
  )

  if command -v node >/dev/null 2>&1; then
    candidates+=("$(command -v node)")
  fi

  local candidate
  for candidate in "${candidates[@]}"; do
    [[ -x "$candidate" ]] || continue
    local major
    major="$(node_major "$candidate" || true)"
    if [[ -n "$major" && "$major" -ge 22 ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  return 1
}

command_name="${GSD_BIN:-gsd}"
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi
if [[ "${1:-}" == "--cli" ]]; then
  command_name="${GSD_BIN:-gsd-cli}"
  shift
fi

node_bin="$(find_node22 || true)"
if [[ -z "$node_bin" ]]; then
  echo "error: GSD 2 requires Node >=22, but no compatible node binary was found." >&2
  echo "hint: install node@22+ or set GSD_NODE_BIN=/absolute/path/to/node." >&2
  exit 1
fi

node_dir="$(cd "$(dirname "$node_bin")" && pwd)"
export PATH="$node_dir:$PATH"

if ! command -v "$command_name" >/dev/null 2>&1; then
  echo "error: $command_name was not found on PATH." >&2
  echo "hint: install gsd-pi or set GSD_BIN=/absolute/path/to/gsd." >&2
  exit 1
fi

exec "$command_name" "$@"
