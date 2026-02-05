#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

COMPOSE_CMD="${COMPOSE_CMD:-docker compose}"

cleanup() {
  $COMPOSE_CMD down -v
}
trap cleanup EXIT

$COMPOSE_CMD up -d redis postgres

export REDIS_URL=redis://localhost:6379
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres_test

cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
INSTA_UPDATE="${INSTA_UPDATE:-}" cargo test --all-features --all
