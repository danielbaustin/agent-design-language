#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/clean_coverage_artifacts.sh [--dry-run] [--include-worktrees]

Deletes local generated coverage artifacts that should not be committed:
  - adl/coverage-summary.json
  - adl/coverage-summary.txt
  - adl/lcov.info
  - loose *.profraw and *.profdata files outside target/

By default this cleans the current checkout only. Use --include-worktrees to
also clean stale local worktree checkouts under .worktrees/.
USAGE
}

dry_run=false
include_worktrees=false

while [ "$#" -gt 0 ]; do
  case "$1" in
    --dry-run)
      dry_run=true
      ;;
    --include-worktrees)
      include_worktrees=true
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "clean-coverage-artifacts: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

repo_root="$(git rev-parse --show-toplevel)"

delete_file() {
  local path="$1"
  if [ "$dry_run" = true ]; then
    printf 'would remove %s\n' "$path"
  else
    rm -f -- "$path"
    printf 'removed %s\n' "$path"
  fi
}

clean_checkout() {
  local checkout_root="$1"

  for path in \
    "$checkout_root/adl/coverage-summary.json" \
    "$checkout_root/adl/coverage-summary.txt" \
    "$checkout_root/adl/lcov.info"
  do
    [ -f "$path" ] && delete_file "$path"
  done

  find "$checkout_root" \
    \( \
      -path "$checkout_root/.git" \
      -o -path "$checkout_root/.worktrees" \
      -o -path "$checkout_root/target" \
      -o -path "$checkout_root/adl/target" \
    \) -prune \
    -o -type f \( -name '*.profraw' -o -name '*.profdata' \) -print |
    while IFS= read -r path; do
      delete_file "$path"
    done
}

clean_checkout "$repo_root"

if [ "$include_worktrees" = true ] && [ -d "$repo_root/.worktrees" ]; then
  find "$repo_root/.worktrees" -mindepth 1 -maxdepth 1 -type d -print |
    while IFS= read -r worktree; do
      clean_checkout "$worktree"
    done
fi
