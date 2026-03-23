#!/usr/bin/env bash
set -euo pipefail

CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

warn() { echo "WARN: $*"; }
info() { echo "INFO: $*"; }
fail() { echo "FAIL: $*" >&2; exit 1; }

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

primary_checkout_root() {
  card_primary_checkout_root
}

default_worktree_path_for_issue() {
  local issue="$1"
  local primary parent
  primary="$(primary_checkout_root)"
  parent="$(cd "$primary/.." && pwd -P)"
  echo "$parent/adl-wp-${issue}"
}

branch_checked_out_worktree_path() {
  local target_branch="$1"
  local wt="" br=""
  while IFS= read -r line; do
    case "$line" in
      worktree\ *)
        wt="${line#worktree }"
        br=""
        ;;
      branch\ refs/heads/*)
        br="${line#branch refs/heads/}"
        if [[ "$br" == "$target_branch" && -n "$wt" ]]; then
          echo "$wt"
          return 0
        fi
        ;;
    esac
  done < <(git worktree list --porcelain)
  return 1
}

main() {
  require_cmd git
  require_cmd gh
  require_cmd awk
  require_cmd sed

  git rev-parse --show-toplevel >/dev/null 2>&1 || fail "not in a git repository"

  local primary repo_status
  primary="$(primary_checkout_root)"
  info "primary checkout: $primary"

  repo_status="$(git -C "$primary" status --porcelain 2>/dev/null || true)"
  if [[ -n "$repo_status" ]]; then
    warn "primary checkout is dirty; start/finish may refuse branch/worktree transitions"
  else
    info "primary checkout is clean"
  fi

  if git rev-parse --verify --quiet origin/main >/dev/null; then
    info "origin/main visible"
  else
    warn "origin/main not visible until fetch (non-fatal for smoke)"
  fi

  local issue slug branch worktree cards_root input output
  issue="999"
  slug="test-smoke"
  branch="codex/${issue}-${slug}"
  worktree="$(default_worktree_path_for_issue "$issue")"
  cards_root="$(cards_root_resolve)"
  input="$cards_root/${issue}/input_${issue}.md"
  output="$cards_root/${issue}/output_${issue}.md"

  info "expected branch: $branch"
  info "expected worktree: $worktree"
  info "expected cards: $input | $output"

  if git show-ref --verify --quiet "refs/heads/$branch"; then
    info "branch exists locally"
    local upstream
    upstream="$(git for-each-ref --format='%(upstream:short)' "refs/heads/$branch" 2>/dev/null || true)"
    if [[ "$upstream" != "origin/main" ]]; then
      warn "branch upstream is '${upstream:-<none>}' (not origin/main); optional remediation: git branch --set-upstream-to=origin/main $branch"
    fi
  else
    info "branch does not yet exist locally"
  fi

  local branch_wt
  branch_wt="$(branch_checked_out_worktree_path "$branch" || true)"
  if [[ -n "$branch_wt" ]]; then
    info "branch is currently checked out at worktree: $branch_wt"
  else
    info "branch is not currently checked out in any worktree"
  fi

  if [[ -e "$worktree" ]]; then
    if git -C "$worktree" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
      info "target worktree path already exists as a git worktree"
    else
      warn "target worktree path exists but is not a git worktree: $worktree"
    fi
  else
    info "target worktree path is free"
  fi

  info "smoke preflight complete (no git state was modified)"
}

main "$@"
