#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  swarm/tools/worktree_prune.sh [--repo <path>] [--managed-root <path>] [--codex-root <path>] [--apply]

Dry-run by default. Removes only clearly safe cases:
- stale git worktree registrations (via `git worktree prune`)
- clean managed worktrees whose branch is already merged into main

Everything else remains report-only.
EOF
}

die() { echo "❌ $*" >&2; exit 1; }

repo=""
managed_root=""
codex_root=""
mode="dry-run"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo) repo="${2-}"; shift 2 ;;
    --managed-root) managed_root="${2-}"; shift 2 ;;
    --codex-root) codex_root="${2-}"; shift 2 ;;
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

args=(--repo "$repo" --format tsv)
[[ -n "$managed_root" ]] && args+=(--managed-root "$managed_root")
[[ -n "$codex_root" ]] && args+=(--codex-root "$codex_root")

rows=()
while IFS= read -r line; do
  rows+=("$line")
done < <("$doctor" "${args[@]}")

declare -a remove_registered
prune_needed="no"

for row in "${rows[@]}"; do
  IFS='|' read -r kind fate path branch clean merged prunable notes <<<"$row"
  case "$fate" in
    prune_now)
      prune_needed="yes"
      ;;
    remove_merged_clean)
      remove_registered+=("$path")
      ;;
  esac
done

echo "Mode: $mode"
echo "Repo: $repo"
echo "Registered clean merged worktrees removable: ${#remove_registered[@]}"
echo "Stale/prunable registrations present: $prune_needed"
echo

if [[ ${#remove_registered[@]} -gt 0 ]]; then
  echo "Registered removals:"
  printf '  %s\n' "${remove_registered[@]}"
  echo
fi

if [[ "$prune_needed" == "yes" ]]; then
  echo "Stale registrations will be cleaned by: git worktree prune --verbose"
  echo
fi

if [[ "$mode" == "dry-run" ]]; then
  echo "Dry run only. Re-run with --apply to execute."
  exit 0
fi

echo "Applying cleanup..."

for path in "${remove_registered[@]}"; do
  echo "git -C $repo worktree remove $path"
  git -C "$repo" worktree remove "$path"
done

if [[ "$prune_needed" == "yes" ]]; then
  echo "git -C $repo worktree prune --verbose"
  git -C "$repo" worktree prune --verbose
fi
