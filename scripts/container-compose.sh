#!/usr/bin/env bash
set -euo pipefail

runtime="${CONTAINER_RUNTIME:-}"

if [[ -n "$runtime" ]]; then
  case "$runtime" in
    docker)
      exec docker compose "$@"
      ;;
    podman)
      exec podman compose "$@"
      ;;
    podman-compose)
      exec podman-compose "$@"
      ;;
    *)
      echo "Unsupported CONTAINER_RUNTIME: $runtime" >&2
      exit 1
      ;;
  esac
fi

if command -v docker >/dev/null 2>&1; then
  exec docker compose "$@"
fi

if command -v podman >/dev/null 2>&1; then
  exec podman compose "$@"
fi

if command -v podman-compose >/dev/null 2>&1; then
  exec podman-compose "$@"
fi

echo "No supported container compose runtime found. Install Docker, Podman, or podman-compose, or set CONTAINER_RUNTIME." >&2
exit 1
