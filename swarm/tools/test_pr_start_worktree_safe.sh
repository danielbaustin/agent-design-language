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

origin="$tmpdir/origin.git"
repo="$tmpdir/repo"
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
  git branch -M main
  git init --bare -q "$origin"
  git remote add origin "$origin"
  git push -q -u origin main
  git fetch -q origin main
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

(
  cd "$repo"

  out1="$("$BASH_BIN" swarm/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "WORKTREE" "$out1" "start prints worktree"
  assert_contains "BRANCH codex/999-test-smoke" "$out1" "start prints branch"
  [[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ]] || {
    echo "assertion failed: primary checkout should remain on main" >&2
    exit 1
  }

  wt_path="$tmpdir/adl-wp-999"
  [[ -d "$wt_path" ]] || {
    echo "assertion failed: expected worktree to exist at $wt_path" >&2
    exit 1
  }
  wt_path="$(cd "$wt_path" && pwd -P)"
  [[ "$(git -C "$wt_path" rev-parse --abbrev-ref HEAD)" == "codex/999-test-smoke" ]] || {
    echo "assertion failed: expected branch in worktree" >&2
    exit 1
  }
  [[ -f "$repo/.adl/cards/999/input_999.md" ]] || {
    echo "assertion failed: expected input card" >&2
    exit 1
  }
  [[ -f "$repo/.adl/cards/999/output_999.md" ]] || {
    echo "assertion failed: expected output card" >&2
    exit 1
  }

  out2="$("$BASH_BIN" swarm/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "Reusing existing worktree for branch: $wt_path" "$out2" "start idempotent worktree reuse"
  [[ "$(git rev-parse --abbrev-ref HEAD)" == "main" ]] || {
    echo "assertion failed: primary checkout should remain on main after rerun" >&2
    exit 1
  }

  git branch --unset-upstream codex/999-test-smoke
  out3="$("$BASH_BIN" swarm/tools/pr.sh start 999 --slug test-smoke --no-fetch-issue)"
  assert_contains "Warning: branch 'codex/999-test-smoke' upstream is '<none>'" "$out3" "upstream warning"

  git branch codex/998-collision origin/main
  git worktree add -q "$tmpdir/other-path" codex/998-collision
  set +e
  bad="$("$BASH_BIN" swarm/tools/pr.sh start 998 --slug collision --no-fetch-issue 2>&1)"
  status=$?
  set -e
  [[ "$status" -ne 0 ]] || {
    echo "assertion failed: expected branch/worktree collision to fail" >&2
    exit 1
  }
  assert_contains "already checked out in worktree" "$bad" "collision message"
  assert_contains "git worktree remove" "$bad" "collision remediation"
)

echo "pr.sh start worktree-safe/idempotent flows: ok"
