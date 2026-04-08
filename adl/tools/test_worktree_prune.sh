#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOCTOR_SRC="$ROOT_DIR/adl/tools/worktree_doctor.sh"
PRUNE_SRC="$ROOT_DIR/adl/tools/worktree_prune.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
managed_root="$tmpdir/managed"
mkdir -p "$repo/adl/tools" "$managed_root"
cp "$DOCTOR_SRC" "$repo/adl/tools/worktree_doctor.sh"
cp "$PRUNE_SRC" "$repo/adl/tools/worktree_prune.sh"
chmod +x "$repo/adl/tools/worktree_doctor.sh" "$repo/adl/tools/worktree_prune.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "base" > README.md
  git add README.md
  git commit -q -m "init"
  git branch -M main
)

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected to find '$pattern'" >&2
    echo "$text" >&2
    exit 1
  }
}

assert_path_missing() {
  local path="$1" label="$2"
  [[ ! -e "$path" ]] || {
    echo "assertion failed ($label): expected path to be removed: $path" >&2
    exit 1
  }
}

managed_root="$(cd "$managed_root" && pwd -P)"

(
  cd "$repo"
  git branch codex/900-merged >/dev/null 2>&1 || true
  git worktree add -q "$managed_root/adl-wp-900" codex/900-merged

  out="$("$BASH_BIN" adl/tools/worktree_prune.sh --repo "$repo" --managed-root "$managed_root" --apply)"
  assert_contains "Registered clean merged worktrees removable: 1" "$out" "empty remove_dirs apply run"
  assert_contains "Directory removals eligible: 0" "$out" "empty remove_dirs apply run"
  assert_contains "Applying cleanup" "$out" "empty remove_dirs apply run"
  assert_path_missing "$managed_root/adl-wp-900" "registered worktree removed"

  mkdir -p "$managed_root/rogue-clean"
  out2="$("$BASH_BIN" adl/tools/worktree_prune.sh --repo "$repo" --managed-root "$managed_root" --apply)"
  assert_contains "Registered clean merged worktrees removable: 0" "$out2" "non-empty remove_dirs apply run"
  assert_contains "Directory removals eligible: 1" "$out2" "non-empty remove_dirs apply run"
  assert_contains "Applying cleanup" "$out2" "non-empty remove_dirs apply run"
  assert_path_missing "$managed_root/rogue-clean" "managed scratch removed"
)

echo "worktree prune apply regression: ok"
