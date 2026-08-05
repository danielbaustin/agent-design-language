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

issue_states_file="$(mktemp)"
trap 'rm -f "$issue_states_file"' EXIT

extract_issue_number() {
  local path="$1" branch="$2" base
  base="$(basename "$path")"
  if [[ "$base" =~ ^adl-wp-([0-9]+)([^0-9].*)?$ ]]; then
    echo "${BASH_REMATCH[1]}"
    return 0
  fi
  if [[ "$branch" =~ ^codex/([0-9]+)- ]]; then
    echo "${BASH_REMATCH[1]}"
    return 0
  fi
  echo ""
}

load_issue_states() {
  local source_json="" issue_json_tmp issue_json_pid waited timeout_seconds
  if [[ -n "${ADL_WORKTREE_PRUNE_ISSUE_STATES_FILE:-}" ]]; then
    source_json="$(cat "$ADL_WORKTREE_PRUNE_ISSUE_STATES_FILE")"
  elif command -v gh >/dev/null 2>&1; then
    timeout_seconds="${ADL_WORKTREE_PRUNE_ISSUE_LIST_TIMEOUT_SECONDS:-15}"
    issue_json_tmp="$(mktemp)"
    (cd "$repo" && gh issue list --state all --limit 1000 --json number,state) >"$issue_json_tmp" 2>/dev/null &
    issue_json_pid="$!"
    waited=0
    while kill -0 "$issue_json_pid" >/dev/null 2>&1; do
      if (( waited >= timeout_seconds )); then
        kill "$issue_json_pid" >/dev/null 2>&1 || true
        wait "$issue_json_pid" >/dev/null 2>&1 || true
        rm -f "$issue_json_tmp"
        return 0
      fi
      sleep 1
      waited=$((waited + 1))
    done
    wait "$issue_json_pid" >/dev/null 2>&1 || true
    source_json="$(cat "$issue_json_tmp")"
    rm -f "$issue_json_tmp"
  fi
  [[ -n "$source_json" ]] || return 0
  ADL_WORKTREE_PRUNE_ISSUE_JSON="$source_json" python3 - "$issue_states_file" <<'PY'
import json
import os
import re
import sys

out = sys.argv[1]
raw = os.environ.get("ADL_WORKTREE_PRUNE_ISSUE_JSON", "")
match = re.search(r"(\[\s*\{.*\}\s*\])", raw, flags=re.S)
if not match:
    raise SystemExit(0)
try:
    issues = json.loads(match.group(1))
except json.JSONDecodeError:
    raise SystemExit(0)
with open(out, "w", encoding="utf-8") as handle:
    for issue in issues:
        number = issue.get("number")
        state = issue.get("state")
        if isinstance(state, str):
            state = state.lower()
        if number is None or state not in {"open", "closed"}:
            continue
        handle.write(f"{number}|{state}\n")
PY
}

issue_state_for() {
  local issue="$1"
  [[ -n "$issue" ]] || {
    echo "unknown"
    return 0
  }
  awk -F'|' -v issue="$issue" '$1 == issue {print $2; found=1; exit} END {if (!found) print "unknown"}' "$issue_states_file"
}

dirty_summary() {
  local path="$1" summary dirty_paths generated_count meaningful_count total_count dirty_path dirty_class
  if [[ ! -d "$path" ]] || ! git -C "$path" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    echo "dirty_class=unknown;dirty_paths=unavailable"
    return 0
  fi
  dirty_paths="$(git -C "$path" status --short 2>/dev/null | sed 's/^...//')"
  summary="$(git -C "$path" status --short 2>/dev/null | head -n 5 | sed 's/[[:space:]]\+/ /g' | paste -sd ';' -)"
  if [[ -z "$summary" ]]; then
    echo "dirty_class=unknown;dirty_paths=unavailable"
  else
    generated_count=0
    meaningful_count=0
    total_count=0
    while IFS= read -r dirty_path; do
      [[ -n "$dirty_path" ]] || continue
      total_count=$((total_count + 1))
      case "$dirty_path" in
        .tmp/*|tmp/*|target/*|adl/target/*|Library/*|UserSettings/*|.adl/logs/*|.adl/runtime/*/logs/*|*.log|*.tmp)
          generated_count=$((generated_count + 1))
          ;;
        *)
          meaningful_count=$((meaningful_count + 1))
          ;;
      esac
    done <<<"$dirty_paths"
    if (( total_count == 0 )); then
      dirty_class="unknown"
    elif (( generated_count == total_count )); then
      dirty_class="generated_disposable_residue"
    elif (( meaningful_count == total_count )); then
      dirty_class="meaningful_unpublished_edits"
    else
      dirty_class="mixed_generated_and_meaningful"
    fi
    echo "dirty_class=$dirty_class;dirty_paths=$summary"
  fi
}

declare -a remove_registered remove_dirs
prune_needed="no"
declare -a selected_rows report_rows skipped_rows open_issue_rows closed_clean_rows closed_dirty_rows

load_issue_states

for row in "${rows[@]}"; do
  IFS='|' read -r kind fate path branch clean merged prunable notes <<<"$row"
  report_rows+=("$row")
  issue_number="$(extract_issue_number "$path" "$branch")"
  issue_state="$(issue_state_for "$issue_number")"
  if [[ "$issue_state" == "open" ]]; then
    open_issue_rows+=("$row|$issue_number|$issue_state|open_issue_not_pruned")
  fi
  case "$fate" in
    prune_now)
      prune_needed="yes"
      selected_rows+=("$row")
      ;;
    remove_merged_clean)
      if [[ "$kind" == "managed_registered" ]]; then
        if [[ "$issue_state" == "open" ]]; then
          skipped_rows+=("$row|open_issue_not_pruned")
        else
          remove_registered+=("$path")
          selected_rows+=("$row")
          if [[ "$issue_state" == "closed" ]]; then
            closed_clean_rows+=("$row|$issue_number|$issue_state")
          fi
        fi
      else
        skipped_rows+=("$row|excluded_by_default")
      fi
      ;;
    backup_then_remove|keep_dirty_active)
      if [[ "$clean" == "dirty" && "$issue_state" == "closed" ]]; then
        closed_dirty_rows+=("$row|$issue_number|$issue_state|$(dirty_summary "$path")")
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
    echo "- open_issue_worktrees_not_pruned: ${#open_issue_rows[@]}"
    echo "- closed_clean_prune_candidates: ${#closed_clean_rows[@]}"
    echo "- closed_dirty_worktrees_needing_disposition: ${#closed_dirty_rows[@]}"
    echo
    echo "## Open Issue Worktrees (Not Pruned)"
    if (( ${#open_issue_rows[@]} == 0 )); then
      echo "- none"
    else
      for row in "${open_issue_rows[@]}"; do
        IFS='|' read -r kind fate path branch clean merged prunable notes issue_number issue_state reason <<<"$row"
        echo "- $path (issue: #$issue_number; state: $issue_state; clean: $clean; merged: $merged; reason: $reason)"
      done
    fi
    echo
    echo "## Closed Clean Prune Candidates"
    if (( ${#closed_clean_rows[@]} == 0 )); then
      echo "- none"
    else
      for row in "${closed_clean_rows[@]}"; do
        IFS='|' read -r kind fate path branch clean merged prunable notes issue_number issue_state <<<"$row"
        echo "- $path (issue: #$issue_number; state: $issue_state; fate: $fate)"
      done
    fi
    echo
    echo "## Closed Dirty Worktrees (Manual Disposition Required)"
    if (( ${#closed_dirty_rows[@]} == 0 )); then
      echo "- none"
    else
      for row in "${closed_dirty_rows[@]}"; do
        IFS='|' read -r kind fate path branch clean merged prunable notes issue_number issue_state summary <<<"$row"
        echo "- $path (issue: #$issue_number; state: $issue_state; fate: $fate; $summary)"
      done
    fi
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
echo "Open issue worktrees not pruned: ${#open_issue_rows[@]}"
echo "Closed clean prune candidates: ${#closed_clean_rows[@]}"
echo "Closed dirty worktrees needing disposition: ${#closed_dirty_rows[@]}"
echo

if (( ${#open_issue_rows[@]} > 0 )); then
  echo "Open issue worktrees (not pruned):"
  for row in "${open_issue_rows[@]}"; do
    IFS='|' read -r kind fate path branch clean merged prunable notes issue_number issue_state reason <<<"$row"
    printf '  %s (#%s %s; %s)\n' "$path" "$issue_number" "$issue_state" "$reason"
  done
  echo
fi

if (( ${#closed_dirty_rows[@]} > 0 )); then
  echo "Closed dirty worktrees (manual disposition required):"
  for row in "${closed_dirty_rows[@]}"; do
    IFS='|' read -r kind fate path branch clean merged prunable notes issue_number issue_state summary <<<"$row"
    printf '  %s (#%s %s; %s)\n' "$path" "$issue_number" "$issue_state" "$summary"
  done
  echo
fi

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
