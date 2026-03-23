#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  adl/tools/worktree_doctor.sh [--repo <path>] [--managed-root <path>] [--codex-root <path>] [--format text|tsv]

Reports the status and recommended fate of ADL worktrees without deleting anything.
Default format is text.
EOF
}

die() { echo "❌ $*" >&2; exit 1; }

format="text"
repo=""
managed_root=""
codex_root=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo) repo="${2-}"; shift 2 ;;
    --managed-root) managed_root="${2-}"; shift 2 ;;
    --codex-root) codex_root="${2-}"; shift 2 ;;
    --format) format="${2-}"; shift 2 ;;
    --help|-h) usage; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

[[ -z "$repo" ]] && repo="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[[ -n "$repo" ]] || die "unable to determine repo root; use --repo <path>"
repo="$(cd "$repo" && pwd -P)"

primary="$repo"
parent="$(cd "$primary/.." && pwd -P)"
if [[ -z "$managed_root" ]]; then
  if [[ "$primary" == "$HOME/git/"* || "$primary" == "$HOME/git" ]]; then
    managed_root="$HOME/git"
  else
    managed_root="$parent"
  fi
fi
[[ -z "$codex_root" ]] && codex_root="$HOME/.codex/worktrees"
[[ -d "$managed_root" ]] && managed_root="$(cd "$managed_root" && pwd -P)"
[[ -d "$codex_root" ]] && codex_root="$(cd "$codex_root" && pwd -P)"

tmp_worktrees="$(mktemp)"
tmp_registered="$(mktemp)"
tmp_rows="$(mktemp)"
trap 'rm -f "$tmp_worktrees" "$tmp_registered" "$tmp_rows"' EXIT

git -C "$repo" worktree list --porcelain > "$tmp_worktrees"

is_foreign_excluded_dir() {
  local path="$1" base num
  base="$(basename "$path")"
  [[ "$base" =~ ^adl-wp-([0-9]+)$ ]] || return 1
  num="${BASH_REMATCH[1]}"
  (( num >= 2 && num <= 24 ))
}

branch_short() {
  local raw="$1"
  if [[ "$raw" == refs/heads/* ]]; then
    echo "${raw#refs/heads/}"
  else
    echo "$raw"
  fi
}

is_head_merged_into_main() {
  local head="$1"
  git -C "$repo" merge-base --is-ancestor "$head" main >/dev/null 2>&1
}

is_branch_merged_into_main() {
  local branch="$1"
  git -C "$repo" merge-base --is-ancestor "$branch" main >/dev/null 2>&1
}

path_clean_state() {
  local path="$1"
  if [[ ! -e "$path" ]]; then
    echo "missing"
  elif git -C "$path" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    if [[ -z "$(git -C "$path" status --porcelain 2>/dev/null)" ]]; then
      echo "clean"
    else
      echo "dirty"
    fi
  else
    echo "not_git"
  fi
}

emit_row() {
  local kind="$1" fate="$2" path="$3" branch="$4" clean="$5" merged="$6" prunable="$7" notes="$8"
  printf '%s|%s|%s|%s|%s|%s|%s|%s\n' "$kind" "$fate" "$path" "$branch" "$clean" "$merged" "$prunable" "$notes" >> "$tmp_rows"
}

path="" head="" branch="" prunable="no" detached="no"
while IFS= read -r line || [[ -n "$line" ]]; do
  if [[ -z "$line" ]]; then
    if [[ -n "$path" ]]; then
      printf '%s\n' "$path" >> "$tmp_registered"
      local_branch="$(branch_short "$branch")"
      clean="$(path_clean_state "$path")"
      merged="unknown"
      kind="other_registered"
      fate="review_other"
      notes=""

      if [[ "$path" == "$primary" ]]; then
        kind="primary_checkout"
        fate="keep_primary"
        notes="canonical_repo_root"
      elif [[ "$prunable" == "yes" ]]; then
        kind="stale_registration"
        fate="prune_now"
        clean="missing"
        notes="git_worktree_metadata_only"
      elif [[ -n "$codex_root" && "$path" == "$codex_root"* ]]; then
        kind="codex_ephemeral"
        fate="ignore_ephemeral"
        if [[ "$detached" == "yes" ]]; then
          if is_head_merged_into_main "$head"; then merged="yes"; else merged="no"; fi
        elif [[ -n "$local_branch" ]]; then
          if is_branch_merged_into_main "$local_branch"; then merged="yes"; else merged="no"; fi
        fi
        notes="outside_managed_adl_namespace"
      elif [[ "$path" == "$managed_root"/adl-wp-* || "$path" == "$managed_root"/adl-lane-* ]]; then
        kind="managed_registered"
        if [[ "$detached" == "yes" ]]; then
          if is_head_merged_into_main "$head"; then merged="yes"; else merged="no"; fi
        elif [[ -n "$local_branch" ]]; then
          if is_branch_merged_into_main "$local_branch"; then merged="yes"; else merged="no"; fi
        fi
        if [[ "$merged" == "yes" && "$clean" == "clean" ]]; then
          fate="remove_merged_clean"
        elif [[ "$merged" == "yes" && "$clean" == "dirty" ]]; then
          fate="backup_then_remove"
        elif [[ "$clean" == "dirty" ]]; then
          fate="keep_dirty_active"
        else
          fate="keep_active"
        fi
        notes="managed_root"
      elif [[ "$path" == /private/tmp/adl-wp-* || "$path" == /tmp/adl-wp-* ]]; then
        kind="temporary_registered"
        fate="prune_now"
        notes="temporary_worktree_namespace"
      fi

      emit_row "$kind" "$fate" "$path" "${local_branch:-DETACHED}" "$clean" "$merged" "$prunable" "$notes"
    fi
    path="" head="" branch="" prunable="no" detached="no"
    continue
  fi

  case "$line" in
    worktree\ *) path="${line#worktree }" ;;
    HEAD\ *) head="${line#HEAD }" ;;
    branch\ *) branch="${line#branch }" ;;
    detached) detached="yes" ;;
    prunable\ *) prunable="yes" ;;
  esac
done < "$tmp_worktrees"

for pattern in "$managed_root"/adl-wp-* "$managed_root"/adl-lane-*; do
  [[ -e "$pattern" ]] || continue
  path="$(cd "$pattern" && pwd -P)"
  if rg -Fxq "$path" "$tmp_registered"; then
    continue
  fi

  clean="$(path_clean_state "$path")"
  branch="$(git -C "$path" symbolic-ref --quiet --short HEAD 2>/dev/null || echo DETACHED)"
  merged="unknown"
  kind="orphan_dir"
  fate="review_orphan"
  notes="not_registered_in_git_worktree_list"

  if is_foreign_excluded_dir "$path"; then
    kind="foreign_excluded"
    fate="ignore_foreign"
    notes="excluded_non_adl_project_namespace"
  elif [[ "$clean" == "dirty" ]]; then
    fate="backup_then_remove"
  elif [[ "$clean" == "clean" ]]; then
    fate="review_orphan_clean"
  fi

  emit_row "$kind" "$fate" "$path" "$branch" "$clean" "$merged" "no" "$notes"
done

if [[ "$format" == "tsv" ]]; then
  sort "$tmp_rows"
  exit 0
fi

echo "WORKTREE_STATUS_REPORT"
echo "repo=$repo"
echo "managed_root=$managed_root"
echo "codex_root=$codex_root"
echo
echo "kind|fate|path|branch|clean|merged|prunable|notes"
sort "$tmp_rows"
echo
echo "SUMMARY"
awk -F'|' '{count[$2]++} END {for (k in count) print k "|" count[k]}' "$tmp_rows" | sort
