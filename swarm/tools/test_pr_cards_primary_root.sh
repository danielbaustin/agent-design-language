#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/swarm/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/swarm/tools/card_paths.sh"
INPUT_TPL_SRC="$ROOT_DIR/swarm/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/swarm/templates/cards/output_card_template.md"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
worktree="$tmpdir/repo-worktree"
mkdir -p "$repo/swarm/tools" "$repo/swarm/templates/cards"
cp "$PR_SH_SRC" "$repo/swarm/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/swarm/tools/card_paths.sh"
cp "$INPUT_TPL_SRC" "$repo/swarm/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/swarm/templates/cards/output_card_template.md"
chmod +x "$repo/swarm/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
  git branch -m main
  git worktree add -q -b codex/301-linked-worktree "$worktree" main
)

(
  cd "$worktree"
  "$BASH_BIN" swarm/tools/pr.sh card 301 --no-fetch-issue --slug linked-worktree >/dev/null
  "$BASH_BIN" swarm/tools/pr.sh output 301 --no-fetch-issue --slug linked-worktree >/dev/null
)

[[ -f "$repo/.adl/cards/301/input_301.md" ]] || { echo "assertion failed: expected primary checkout input card" >&2; exit 1; }
[[ -f "$repo/.adl/cards/301/output_301.md" ]] || { echo "assertion failed: expected primary checkout output card" >&2; exit 1; }
[[ ! -f "$worktree/.adl/cards/301/input_301.md" ]] || { echo "assertion failed: unexpected linked worktree input card" >&2; exit 1; }
[[ ! -f "$worktree/.adl/cards/301/output_301.md" ]] || { echo "assertion failed: unexpected linked worktree output card" >&2; exit 1; }

echo "pr.sh linked worktree cards root resolution: ok"
