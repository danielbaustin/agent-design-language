#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools"
cp "$ROOT_DIR/adl/tools/pr.sh" "$repo/adl/tools/pr.sh"
cp "$ROOT_DIR/adl/tools/card_paths.sh" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  touch README.md
  git add README.md
  git commit -q -m "init"
)

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "actual output:" >&2
    echo "$text" >&2
    exit 1
  }
}

set +e
out="$(
  cd "$repo" &&
  "$BASH_BIN" adl/tools/pr.sh create --title "retired path" 2>&1
)"
status=$?
set -e

[[ "$status" -ne 0 ]] || {
  echo "assertion failed: expected pr.sh create to be unavailable" >&2
  exit 1
}

assert_contains 'Unknown command: create' "$out" "unknown command"

echo "pr.sh create removal: ok"
