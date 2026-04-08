#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TMPDIR_ROOT="$(mktemp -d)"
trap 'rm -rf "$TMPDIR_ROOT"' EXIT

OUT_DIR="$TMPDIR_ROOT/review_surface"
MANIFEST="$OUT_DIR/demo_manifest.json"
README="$OUT_DIR/README.md"

bash adl/tools/demo_v0871_review_surface.sh "$OUT_DIR" >/dev/null

[[ -f "$MANIFEST" ]] || {
  echo "assertion failed: review manifest missing" >&2
  exit 1
}
[[ -f "$README" ]] || {
  echo "assertion failed: review README missing" >&2
  exit 1
}

grep -F '"review_surface_version": "adl.runtime_review_surface.v1"' "$MANIFEST" >/dev/null || {
  echo "assertion failed: manifest version missing" >&2
  exit 1
}
grep -F '"demo_id": "D8"' "$MANIFEST" >/dev/null || {
  echo "assertion failed: D8 identity missing" >&2
  exit 1
}
grep -F '"demo_id": "D6"' "$MANIFEST" >/dev/null || {
  echo "assertion failed: D6 package missing" >&2
  exit 1
}
grep -F '"demo_id": "D7"' "$MANIFEST" >/dev/null || {
  echo "assertion failed: D7 package missing" >&2
  exit 1
}
grep -F 'Review D6 first for the canonical operator entrypoint.' "$README" >/dev/null || {
  echo "assertion failed: README walkthrough missing D6 guidance" >&2
  exit 1
}
grep -F 'Then inspect D7 for persistence, pause-state, and continuity evidence.' "$README" >/dev/null || {
  echo "assertion failed: README walkthrough missing D7 guidance" >&2
  exit 1
}

cargo run --quiet --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin adl -- \
  tooling review-runtime-surface --review-root "$OUT_DIR" >/dev/null

echo "demo_v0871_review_surface: ok"
