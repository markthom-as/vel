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

resolve_gsd_package_root() {
  "$node_bin" - "$1" <<'NODE'
const fs = require('fs');
const path = require('path');

let current = fs.realpathSync(process.argv[2]);
if (!fs.statSync(current).isDirectory()) {
  current = path.dirname(current);
}

while (true) {
  const packageJsonPath = path.join(current, 'package.json');
  if (fs.existsSync(packageJsonPath)) {
    try {
      const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
      if (packageJson.name === 'gsd-pi') {
        process.stdout.write(current);
        process.exit(0);
      }
    } catch {
      // Keep walking; a malformed unrelated package file is not actionable here.
    }
  }

  const parent = path.dirname(current);
  if (parent === current) {
    process.exit(1);
  }
  current = parent;
}
NODE
}

ensure_gsd_internal_links() {
  local command_path package_root scope_dir target link
  command_path="$(command -v "$command_name" || true)"
  [[ -n "$command_path" ]] || return 0

  package_root="$(resolve_gsd_package_root "$command_path" 2>/dev/null || true)"
  [[ -n "$package_root" ]] || return 0

  target="$package_root/packages/mcp-server"
  link="$package_root/node_modules/@gsd-build/mcp-server"
  [[ -d "$target" ]] || return 0
  [[ -e "$link" || -L "$link" ]] && return 0

  scope_dir="$(dirname "$link")"
  mkdir -p "$scope_dir"
  ln -s "$target" "$link"
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

ensure_gsd_internal_links

exec "$command_name" "$@"
