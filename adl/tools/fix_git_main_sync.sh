#!/usr/bin/env bash
set -euo pipefail

snapshot_dir=""

die() {
  echo "fix-git: $*" >&2
  exit 1
}

cleanup() {
  if [[ -n "$snapshot_dir" && -d "$snapshot_dir" ]]; then
    rm -rf "$snapshot_dir"
  fi
}

trap cleanup EXIT HUP INT TERM

capture_local_adl_cards() {
  local cards_root="$repo_root/.adl"
  if [[ ! -d "$cards_root" ]]; then
    return 0
  fi

  snapshot_dir="$(mktemp -d "${TMPDIR:-/tmp}/adl-fix-git.XXXXXX")"
  local preserve_list="$snapshot_dir/preserve.list"
  : >"$preserve_list"

  while IFS= read -r source_path; do
    [[ -n "$source_path" ]] || continue
    local relative_path="${source_path#$repo_root/}"
    local snapshot_path="$snapshot_dir/$relative_path"
    mkdir -p "$(dirname "$snapshot_path")"
    cp "$source_path" "$snapshot_path"
    printf '%s\n' "$relative_path" >>"$preserve_list"
  done < <(
    find "$cards_root" -type f \
      \( -path "$cards_root/*/bodies/*.md" \
      -o -path "$cards_root/*/tasks/*/stp.md" \
      -o -path "$cards_root/*/tasks/*/sip.md" \
      -o -path "$cards_root/*/tasks/*/sor.md" \) |
      sort
  )
}

restore_missing_local_adl_cards() {
  local preserve_list="$snapshot_dir/preserve.list"
  if [[ ! -f "$preserve_list" ]]; then
    return 0
  fi

  while IFS= read -r relative_path; do
    [[ -n "$relative_path" ]] || continue
    local target_path="$repo_root/$relative_path"
    local snapshot_path="$snapshot_dir/$relative_path"
    if [[ ! -e "$target_path" && -f "$snapshot_path" ]]; then
      mkdir -p "$(dirname "$target_path")"
      cp "$snapshot_path" "$target_path"
    fi
  done <"$preserve_list"
}

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" ||
  die "not inside a git checkout"

branch="$(git -C "$repo_root" rev-parse --abbrev-ref HEAD)"
if [[ "$branch" != "main" ]]; then
  die "refusing to switch from '$branch' to main; run this only from an already-clean main checkout"
fi

if [[ -n "$(git -C "$repo_root" status --porcelain)" ]]; then
  die "refusing to pull with local changes in $repo_root"
fi

main_worktree="$(git -C "$repo_root" worktree list --porcelain |
  awk '
    /^worktree / { path = substr($0, 10) }
    /^branch refs\/heads\/main$/ { print path }
  ')"

if [[ -n "$main_worktree" && "$main_worktree" != "$repo_root" ]]; then
  die "main is checked out at $main_worktree; pull from that worktree or remove it intentionally"
fi

capture_local_adl_cards
git -C "$repo_root" fetch origin main
git -C "$repo_root" merge --ff-only origin/main
restore_missing_local_adl_cards
