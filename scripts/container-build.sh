#!/usr/bin/env bash
set -euo pipefail

runtime="${CONTAINER_RUNTIME:-}"
image_tag="${1:-veld:latest}"

if [[ -n "$runtime" ]]; then
  case "$runtime" in
    docker)
      exec docker build -t "$image_tag" .
      ;;
    podman)
      exec podman build -t "$image_tag" .
      ;;
    *)
      echo "Unsupported CONTAINER_RUNTIME for image build: $runtime" >&2
      exit 1
      ;;
  esac
fi

if command -v docker >/dev/null 2>&1; then
  exec docker build -t "$image_tag" .
fi

if command -v podman >/dev/null 2>&1; then
  exec podman build -t "$image_tag" .
fi

echo "No supported container runtime found for image build. Install Docker or Podman, or set CONTAINER_RUNTIME." >&2
exit 1
