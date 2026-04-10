#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PR_SH_SRC="$ROOT_DIR/adl/tools/pr.sh"
CARD_PATHS_SRC="$ROOT_DIR/adl/tools/card_paths.sh"
INPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/input_card_template.md"
OUTPUT_TPL_SRC="$ROOT_DIR/adl/templates/cards/output_card_template.md"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
worktree="$tmpdir/repo-worktree"
origin="$tmpdir/origin.git"
mkdir -p "$repo/adl/tools" "$repo/adl/templates/cards"
cp "$PR_SH_SRC" "$repo/adl/tools/pr.sh"
cp "$CARD_PATHS_SRC" "$repo/adl/tools/card_paths.sh"
cp "$INPUT_TPL_SRC" "$repo/adl/templates/cards/input_card_template.md"
cp "$OUTPUT_TPL_SRC" "$repo/adl/templates/cards/output_card_template.md"
chmod +x "$repo/adl/tools/pr.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  git add -A
  git commit -q -m "init"
  git branch -m main
  git init --bare -q "$origin"
  git remote add origin "$origin"
  git push -q -u origin main
  git fetch -q origin main
  git worktree add -q -b codex/301-linked-worktree "$worktree" origin/main
)

(
  cd "$worktree"
  "$BASH_BIN" adl/tools/pr.sh card 301 --no-fetch-issue --slug linked-worktree >/dev/null
  "$BASH_BIN" adl/tools/pr.sh output 301 --no-fetch-issue --slug linked-worktree >/dev/null
)

[[ -f "$worktree/.adl/v0.86/tasks/issue-0301__linked-worktree/sip.md" ]] || { echo "assertion failed: expected linked worktree canonical input card" >&2; exit 1; }
[[ -f "$worktree/.adl/v0.86/tasks/issue-0301__linked-worktree/sor.md" ]] || { echo "assertion failed: expected linked worktree canonical output card" >&2; exit 1; }
[[ -L "$worktree/.adl/cards/301/input_301.md" ]] || { echo "assertion failed: expected linked worktree input compatibility link" >&2; exit 1; }
[[ -L "$worktree/.adl/cards/301/output_301.md" ]] || { echo "assertion failed: expected linked worktree output compatibility link" >&2; exit 1; }
[[ ! -e "$repo/.adl/v0.86/tasks/issue-0301__linked-worktree/sip.md" ]] || { echo "assertion failed: unexpected primary checkout canonical input card" >&2; exit 1; }
[[ ! -e "$repo/.adl/v0.86/tasks/issue-0301__linked-worktree/sor.md" ]] || { echo "assertion failed: unexpected primary checkout canonical output card" >&2; exit 1; }

echo "pr.sh linked worktree cards root isolation: ok"
