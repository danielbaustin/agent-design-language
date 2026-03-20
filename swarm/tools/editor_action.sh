#!/usr/bin/env bash
set -euo pipefail

CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

die() {
  echo "ERROR: $*" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage:
  swarm/tools/editor_action.sh start --issue <number> --branch codex/<issue>-<slug> [--slug <slug>] [--dry-run]

Purpose:
  Thin editor-adjacent adapter for bounded control-plane actions.

Current actions:
  start    Validate issue/branch pairing and invoke `swarm/tools/pr.sh start`.
USAGE
}

repo_root() {
  git rev-parse --show-toplevel 2>/dev/null || die "Not in a git repo"
}

normalize_issue_or_die() {
  local raw="$1"
  local normalized
  normalized="$(card_issue_normalize "$raw" 2>/dev/null)" || die "invalid issue number: $raw"
  echo "$normalized"
}

derive_slug_from_branch() {
  local issue="$1" branch="$2"
  [[ "$branch" =~ ^codex/([0-9]+)-([a-z0-9][a-z0-9-]*)$ ]] || die "branch must match codex/<issue>-<slug>"
  local branch_issue="${BASH_REMATCH[1]}"
  local slug="${BASH_REMATCH[2]}"
  [[ "$branch_issue" == "$issue" ]] || die "branch issue prefix ($branch_issue) does not match issue number ($issue)"
  echo "$slug"
}

ACTION="${1:-}"
[[ -n "$ACTION" ]] || { usage; exit 1; }
shift || true

case "$ACTION" in
  start) ;;
  -h|--help) usage; exit 0 ;;
  *) die "unsupported action: $ACTION" ;;
esac

ISSUE=""
BRANCH=""
SLUG=""
DRY_RUN=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --issue) ISSUE="$2"; shift 2 ;;
    --branch) BRANCH="$2"; shift 2 ;;
    --slug) SLUG="$2"; shift 2 ;;
    --dry-run) DRY_RUN=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

[[ -n "$ISSUE" ]] || die "--issue is required"
[[ -n "$BRANCH" ]] || die "--branch is required"

ISSUE="$(normalize_issue_or_die "$ISSUE")"

if [[ -z "$SLUG" ]]; then
  SLUG="$(derive_slug_from_branch "$ISSUE" "$BRANCH")"
else
  [[ "$SLUG" =~ ^[a-z0-9][a-z0-9-]*$ ]] || die "slug must match [a-z0-9-]+"
  derive_slug_from_branch "$ISSUE" "$BRANCH" >/dev/null
fi

ROOT="$(repo_root)"
cd "$ROOT"

CMD=(./swarm/tools/pr.sh start "$ISSUE" --slug "$SLUG")

if [[ "$DRY_RUN" == true ]]; then
  printf '%s\n' "${CMD[*]}"
  exit 0
fi

exec "${CMD[@]}"
