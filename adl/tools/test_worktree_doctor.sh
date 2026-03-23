#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DOCTOR_SRC="$ROOT_DIR/adl/tools/worktree_doctor.sh"
PRUNE_SRC="$ROOT_DIR/adl/tools/worktree_prune.sh"
BASH_BIN="$(command -v bash)"

tmpdir="$(mktemp -d)"
tmpdir="$(cd "$tmpdir" && pwd -P)"
trap 'rm -rf "$tmpdir"' EXIT

repo="$tmpdir/repo"
origin="$tmpdir/origin.git"
managed_root="$tmpdir/managed"
codex_root="$tmpdir/codex"
mkdir -p "$repo/adl/tools" "$managed_root" "$codex_root"
cp "$DOCTOR_SRC" "$repo/adl/tools/worktree_doctor.sh"
cp "$PRUNE_SRC" "$repo/adl/tools/worktree_prune.sh"
chmod +x "$repo/adl/tools/worktree_doctor.sh" "$repo/adl/tools/worktree_prune.sh"

(
  cd "$repo"
  git init -q
  git config user.name "Test User"
  git config user.email "test@example.com"
  echo "base" > README.md
  git add -A
  git commit -q -m "init"
  git branch -M main
  git init --bare -q "$origin"
  git remote add origin "$origin"
  git push -q -u origin main
  git fetch -q origin main
)

managed_root="$(cd "$managed_root" && pwd -P)"
codex_root="$(cd "$codex_root" && pwd -P)"

assert_contains() {
  local pattern="$1" text="$2" label="$3"
  grep -Fq "$pattern" <<<"$text" || {
    echo "assertion failed ($label): expected '$pattern'" >&2
    echo "$text" >&2
    exit 1
  }
}

(
  cd "$repo"
  git branch codex/900-merged origin/main >/dev/null 2>&1 || true
  git worktree add -q "$managed_root/adl-wp-900" codex/900-merged

  git branch codex/901-active origin/main >/dev/null 2>&1 || true
  git worktree add -q "$managed_root/adl-wp-901" codex/901-active
  (
    cd "$managed_root/adl-wp-901"
    echo "active" > active.txt
    git add active.txt
    git commit -q -m "active change"
  )

  git branch codex/937-stale origin/main >/dev/null 2>&1 || true
  git worktree add -q "$tmpdir/tmp-adl-wp-937" codex/937-stale
  rm -rf "$tmpdir/tmp-adl-wp-937"

  git worktree add -q --detach "$codex_root/abcd/agent-design-language" main

  mkdir -p "$managed_root/adl-wp-13"
  mkdir -p "$managed_root/adl-wp-473__conflict__20260224_174604"

  out="$("$BASH_BIN" adl/tools/worktree_doctor.sh --repo "$repo" --managed-root "$managed_root" --codex-root "$codex_root" --format tsv)"
  assert_contains "managed_registered|remove_merged_clean|$managed_root/adl-wp-900|" "$out" "merged clean managed worktree"
  assert_contains "managed_registered|keep_active|$managed_root/adl-wp-901|" "$out" "active managed worktree"
  assert_contains "stale_registration|prune_now|$tmpdir/tmp-adl-wp-937|" "$out" "stale registration"
  assert_contains "codex_ephemeral|ignore_ephemeral|$codex_root/abcd/agent-design-language|" "$out" "codex ephemeral"
  assert_contains "foreign_excluded|ignore_foreign|$managed_root/adl-wp-13|" "$out" "foreign excluded"
  assert_contains "orphan_dir|review_orphan|$managed_root/adl-wp-473__conflict__20260224_174604|" "$out" "orphan dir"

  dry="$("$BASH_BIN" adl/tools/worktree_prune.sh --repo "$repo" --managed-root "$managed_root" --codex-root "$codex_root")"
  assert_contains "Registered clean merged worktrees removable: 1" "$dry" "prune dry-run removal count"
  assert_contains "Dry run only" "$dry" "prune dry-run mode"
)

echo "worktree doctor/prune classification: ok"
