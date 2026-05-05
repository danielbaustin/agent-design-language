#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
mkdir -p "$repo/adl/tools" "$repo/adl"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
chmod +x "$repo/adl/tools/pr.sh"
touch "$repo/adl/Cargo.toml"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git config commit.gpgsign false
  echo "seed" > README.md
  git add README.md
  git commit -q -m "init"
)

set +e
(
  cd "$repo"
  ADL_PR_RUST_BIN=/usr/bin/false \
    "$BASH_BIN" adl/tools/pr.sh doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full >/dev/null
)
failure_status=$?
set -e

[[ "$failure_status" -eq 1 ]] || {
  echo "assertion failed: expected delegated Rust failure exit status to propagate, got $failure_status" >&2
  exit 1
}

set +e
(
  cd "$repo"
  ADL_PR_RUST_BIN=/usr/bin/true \
    "$BASH_BIN" adl/tools/pr.sh doctor 1152 --slug rust-start --no-fetch-issue --version v0.86 --mode full >/dev/null
)
success_status=$?
set -e

[[ "$success_status" -eq 0 ]] || {
  echo "assertion failed: expected delegated Rust success exit status to propagate, got $success_status" >&2
  exit 1
}

echo "pr.sh delegated success/failure exit status propagation: ok"
