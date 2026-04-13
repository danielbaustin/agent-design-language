#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  adl/tools/worktree_prune.sh [--repo <path>] [--managed-root <path>] [--codex-root <path>] [--limit <n>] [--report <path>] [--include-legacy-external] [--include-scratch] [--apply]

Dry-run by default. Removes only clearly safe cases:
- stale git worktree registrations (via `git worktree prune`)
- clean repo-local managed worktrees whose branch is already merged into main

Legacy external clones and repo-local scratch directories remain report-only unless
explicitly included.
EOF
}

die() { echo "❌ $*" >&2; exit 1; }

repo=""
managed_root=""
codex_root=""
mode="dry-run"
limit=""
report_path=""
include_legacy_external="no"
include_scratch="no"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo) repo="${2-}"; shift 2 ;;
    --managed-root) managed_root="${2-}"; shift 2 ;;
    --codex-root) codex_root="${2-}"; shift 2 ;;
    --limit) limit="${2-}"; shift 2 ;;
    --report) report_path="${2-}"; shift 2 ;;
    --include-legacy-external) include_legacy_external="yes"; shift ;;
    --include-scratch) include_scratch="yes"; shift ;;
    --apply) mode="apply"; shift ;;
    --help|-h) usage; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
doctor="$script_dir/worktree_doctor.sh"
[[ -x "$doctor" ]] || die "missing executable doctor tool: $doctor"

[[ -z "$repo" ]] && repo="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[[ -n "$repo" ]] || die "unable to determine repo root; use --repo <path>"
if [[ -n "$limit" ]]; then
  [[ "$limit" =~ ^[0-9]+$ ]] || die "--limit must be a non-negative integer"
fi

args=(--repo "$repo" --format tsv)
[[ -n "$managed_root" ]] && args+=(--managed-root "$managed_root")
[[ -n "$codex_root" ]] && args+=(--codex-root "$codex_root")

rows=()
while IFS= read -r line; do
  rows+=("$line")
done < <("$doctor" "${args[@]}")

declare -a remove_registered remove_dirs
prune_needed="no"
declare -a selected_rows report_rows skipped_rows

for row in "${rows[@]}"; do
  IFS='|' read -r kind fate path branch clean merged prunable notes <<<"$row"
  report_rows+=("$row")
  case "$fate" in
    prune_now)
      prune_needed="yes"
      selected_rows+=("$row")
      ;;
    remove_merged_clean)
      if [[ "$kind" == "managed_registered" ]]; then
        remove_registered+=("$path")
        selected_rows+=("$row")
      else
        skipped_rows+=("$row|excluded_by_default")
      fi
      ;;
    remove_legacy_replaced|remove_scratch_clean)
      if [[ "$kind" == "legacy_external_registered" ]]; then
        if [[ "$include_legacy_external" == "yes" ]]; then
          remove_registered+=("$path")
          selected_rows+=("$row")
        else
          skipped_rows+=("$row|excluded_by_default")
        fi
      else
        if [[ "$include_scratch" == "yes" ]]; then
          remove_dirs+=("$path")
          selected_rows+=("$row")
        else
          skipped_rows+=("$row|excluded_by_default")
        fi
      fi
      ;;
  esac
done

limit_array() {
  local max="$1"
  shift
  local count=0
  local item
  for item in "$@"; do
    if [[ -n "$max" ]] && (( count >= max )); then
      break
    fi
    printf '%s\n' "$item"
    count=$((count + 1))
  done
}

declare -a trimmed_registered
trimmed_registered=()
while IFS= read -r line; do
  [[ -n "$line" ]] && trimmed_registered+=("$line")
done < <(limit_array "$limit" "${remove_registered[@]-}")
remove_registered=()
if (( ${#trimmed_registered[@]} > 0 )); then
  remove_registered=("${trimmed_registered[@]}")
fi

remaining_slots=""
if [[ -n "$limit" ]]; then
  remaining_slots=$(( limit - ${#remove_registered[@]} ))
  if (( remaining_slots < 0 )); then
    remaining_slots=0
  fi
fi
if [[ -n "$limit" ]]; then
  declare -a trimmed_dirs
  trimmed_dirs=()
  while IFS= read -r line; do
    [[ -n "$line" ]] && trimmed_dirs+=("$line")
  done < <(limit_array "$remaining_slots" "${remove_dirs[@]-}")
  remove_dirs=()
  if (( ${#trimmed_dirs[@]} > 0 )); then
    remove_dirs=("${trimmed_dirs[@]}")
  fi
fi

selected_registered_count=0
selected_dir_count=0
for _path in "${remove_registered[@]+"${remove_registered[@]}"}"; do
  selected_registered_count=$((selected_registered_count + 1))
done
for _path in "${remove_dirs[@]+"${remove_dirs[@]}"}"; do
  selected_dir_count=$((selected_dir_count + 1))
done

write_report() {
  local report="$1"
  mkdir -p "$(dirname "$report")"
  {
    echo "# Worktree Cleanup Report"
    echo
    echo "- mode: $mode"
    echo "- repo: $repo"
    echo "- managed_root: ${managed_root:-$repo/.worktrees}"
    echo "- include_legacy_external: $include_legacy_external"
    echo "- include_scratch: $include_scratch"
    echo "- limit: ${limit:-all}"
    echo "- registered_removals_selected: $selected_registered_count"
    echo "- directory_removals_selected: $selected_dir_count"
    echo "- stale_registrations_present: $prune_needed"
    echo
    echo "## Selected Registered Removals"
    if (( selected_registered_count == 0 )); then
      echo "- none"
    else
      for path in "${remove_registered[@]+"${remove_registered[@]}"}"; do
        echo "- $path"
      done
    fi
    echo
    echo "## Selected Directory Removals"
    if (( selected_dir_count == 0 )); then
      echo "- none"
    else
      for path in "${remove_dirs[@]+"${remove_dirs[@]}"}"; do
        echo "- $path"
      done
    fi
    echo
    echo "## Selected Actions"
    if (( selected_registered_count == 0 && selected_dir_count == 0 )) && [[ "$prune_needed" != "yes" ]]; then
      echo "- none"
    else
      for path in "${remove_registered[@]+"${remove_registered[@]}"}"; do
        echo "- git worktree remove $path"
      done
      for path in "${remove_dirs[@]+"${remove_dirs[@]}"}"; do
        echo "- rm -rf $path"
      done
      if [[ "$prune_needed" == "yes" ]]; then
        echo "- git worktree prune --verbose"
      fi
    fi
    echo
    echo "## Excluded By Default"
    if (( ${#skipped_rows[@]} == 0 )); then
      echo "- none"
    else
      for row in "${skipped_rows[@]}"; do
        IFS='|' read -r kind fate path branch clean merged prunable notes reason <<<"$row"
        echo "- $path ($fate; $reason)"
      done
    fi
  } >"$report"
}

echo "Mode: $mode"
echo "Repo: $repo"
echo "Legacy external included: $include_legacy_external"
echo "Scratch included: $include_scratch"
echo "Limit: ${limit:-all}"
echo "Registered clean merged worktrees removable: $selected_registered_count"
echo "Directory removals eligible: $selected_dir_count"
echo "Stale/prunable registrations present: $prune_needed"
echo

if (( selected_registered_count > 0 )); then
  echo "Registered removals:"
  for path in "${remove_registered[@]+"${remove_registered[@]}"}"; do
    printf '  %s\n' "$path"
  done
  echo
fi

if (( selected_dir_count > 0 )); then
  echo "Directory removals:"
  for path in "${remove_dirs[@]+"${remove_dirs[@]}"}"; do
    printf '  %s\n' "$path"
  done
  echo
fi

if [[ "$prune_needed" == "yes" ]]; then
  echo "Stale registrations will be cleaned by: git worktree prune --verbose"
  echo
fi

if [[ -n "$report_path" ]]; then
  write_report "$report_path"
  echo "Report: $report_path"
  echo
fi

if [[ "$mode" == "dry-run" ]]; then
  echo "Dry run only. Re-run with --apply to execute."
  exit 0
fi

echo "Applying cleanup..."

if (( selected_registered_count > 0 )); then
  for path in "${remove_registered[@]+"${remove_registered[@]}"}"; do
    echo "git -C $repo worktree remove $path"
    git -C "$repo" worktree remove "$path"
  done
fi

if (( selected_dir_count > 0 )); then
  for path in "${remove_dirs[@]+"${remove_dirs[@]}"}"; do
    echo "rm -rf $path"
    rm -rf "$path"
  done
fi

if [[ "$prune_needed" == "yes" ]]; then
  echo "git -C $repo worktree prune --verbose"
  git -C "$repo" worktree prune --verbose
fi
