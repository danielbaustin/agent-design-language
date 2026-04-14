#!/usr/bin/env bash
set -euo pipefail

snapshot_dir=""
temp_main_worktree=""
repo_root=""

die() {
  echo "fix-git: $*" >&2
  exit 1
}

cleanup() {
  if [[ -n "$snapshot_dir" && -d "$snapshot_dir" ]]; then
    rm -rf "$snapshot_dir"
  fi
  if [[ -n "$temp_main_worktree" && -d "$temp_main_worktree" ]]; then
    git -C "$repo_root" worktree remove --force "$temp_main_worktree" >/dev/null 2>&1 || true
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

latest_local_adl_version() {
  local version_root="$repo_root/.adl"
  [[ -d "$version_root" ]] || return 0
  find "$version_root" -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | \
    sort -V | tail -n 1
}

ensure_local_main_branch() {
  if git -C "$repo_root" show-ref --verify --quiet refs/heads/main; then
    return 0
  fi
  git -C "$repo_root" branch --track main origin/main >/dev/null
}

create_temp_main_worktree() {
  ensure_local_main_branch
  temp_main_worktree="$(mktemp -d "${TMPDIR:-/tmp}/adl-main-sync.XXXXXX")"
  rm -rf "$temp_main_worktree"
  git -C "$repo_root" worktree add "$temp_main_worktree" main >/dev/null
  printf '%s\n' "$temp_main_worktree"
}

run_closeout_catchup() {
  [[ "${ADL_MAIN_SYNC_CLOSEOUT_DISABLE:-0}" == "1" ]] && return 0
  command -v gh >/dev/null 2>&1 || {
    echo "fix-git: gh not found; skipping closeout catch-up" >&2
    return 0
  }

  local versions_csv="${ADL_MAIN_SYNC_CLOSEOUT_VERSIONS:-}"
  local closeout_repo="${ADL_MAIN_SYNC_CLOSEOUT_REPO:-}"
  if [[ -z "$versions_csv" ]]; then
    versions_csv="$(latest_local_adl_version || true)"
  fi
  [[ -n "$versions_csv" ]] || return 0

  local version
  OLDIFS="$IFS"
  IFS=','
  for version in $versions_csv; do
    version="$(echo "$version" | xargs)"
    [[ -n "$version" ]] || continue
    if [[ -n "$closeout_repo" ]]; then
      bash "$repo_root/adl/tools/closeout_completed_issue_wave.sh" --version "$version" --repo "$closeout_repo"
    else
      bash "$repo_root/adl/tools/closeout_completed_issue_wave.sh" --version "$version"
    fi
  done
  IFS="$OLDIFS"
}

repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" ||
  die "not inside a git checkout"

invocation_root="$repo_root"
branch="$(git -C "$invocation_root" rev-parse --abbrev-ref HEAD)"

main_worktree="$(git -C "$repo_root" worktree list --porcelain |
  awk '
    /^worktree / { path = substr($0, 10) }
    /^branch refs\/heads\/main$/ { print path }
  ')"

sync_root="$invocation_root"
if [[ "$branch" != "main" ]]; then
  if [[ -n "$main_worktree" ]]; then
    sync_root="$main_worktree"
  else
    sync_root="$(create_temp_main_worktree)"
  fi
fi

sync_branch="$(git -C "$sync_root" rev-parse --abbrev-ref HEAD)"
if [[ "$sync_branch" != "main" ]]; then
  die "refusing to sync from '$sync_root' because it is on '$sync_branch', not main"
fi

if [[ -n "$(git -C "$sync_root" status --porcelain)" ]]; then
  die "refusing to pull with local changes in $sync_root"
fi

repo_root="$sync_root"
capture_local_adl_cards
git -C "$repo_root" fetch origin main
git -C "$repo_root" merge --ff-only origin/main
restore_missing_local_adl_cards
run_closeout_catchup
