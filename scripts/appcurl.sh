#!/usr/bin/env bash
set -euo pipefail

path="${1:-/}"
shift || true

case "$path" in
  /*) ;;
  *) path="/$path" ;;
esac

exec curl "$@" "http://localhost:5150${path}"
