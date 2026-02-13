#!/usr/bin/env bash
# Re-exec under bash when invoked via sh or another non-bash shell.
if [ -z "${BASH_VERSION:-}" ]; then
  exec bash "$0" "$@"
fi
set -euo pipefail

PR_SH="${PR_SH:-swarm/tools/pr.sh}"
CODEXW_SH="${CODEXW_SH:-swarm/tools/codexw.sh}"
CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
CARD=""
MODE="full-auto"   # full-auto | auto-edit | suggest | help
SLUG=""
PATHS=""

usage() {
  cat <<'USAGE' >&2
Usage:
  swarm/tools/codex_pr.sh <input-card> --paths "<p1,p2,...>" [--mode full-auto|auto-edit|suggest|help] [--slug <slug>] [--pr-sh <path>] [--codexw-sh <path>]

Notes:
- --paths is required.
- --paths '.' is forbidden.
- .adl/cards must not be included in --paths.
- --mode help validates inputs and exits before running codex/pr.
USAGE
}

die() { printf '%s\n' "ERROR: $*" >&2; exit 2; }

# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

codex_cli_preflight() {
  command -v codex >/dev/null 2>&1 || die "codex CLI not found in PATH"
  codex exec --help >/dev/null 2>&1 || die "codex CLI sanity check failed: 'codex exec --help' did not succeed"
}

issue_from_input_path() {
  local p="$1"
  local base
  base="$(basename "$p")"

  if [[ "$p" =~ (^|/)\.adl/cards/([0-9]+)/input_([0-9]+)\.md$ ]]; then
    [[ "${BASH_REMATCH[2]}" == "${BASH_REMATCH[3]}" ]] || die "Card path mismatch: $p"
    card_issue_normalize "${BASH_REMATCH[2]}"
    return 0
  fi

  if [[ "$base" =~ ^issue-([0-9]+)__input__v[0-9.]+\.md$ ]]; then
    card_issue_normalize "${BASH_REMATCH[1]}"
    return 0
  fi

  die "Could not parse input card path: $p (expected .adl/cards/143/input_143.md or issue-0143__input__v0.3.md)"
}

version_from_card() {
  local card="$1"
  awk -F':' '/^Version:/ {gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2); print $2; exit}' "$card" || true
}

working_tree_dirty() {
  ! git diff --quiet || ! git diff --cached --quiet
}

if [[ $# -eq 1 && ( "$1" == "-h" || "$1" == "--help" ) ]]; then
  usage
  exit 0
fi

if [[ $# -lt 1 ]]; then
  usage
  exit 2
fi

CARD="$1"
shift

while [[ $# -gt 0 ]]; do
  case "$1" in
    --paths) PATHS="$2"; shift 2 ;;
    --mode) MODE="$2"; shift 2 ;;
    --slug) SLUG="$2"; shift 2 ;;
    --pr-sh) PR_SH="$2"; shift 2 ;;
    --codexw-sh) CODEXW_SH="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "Unknown arg: $1" ;;
  esac
done

if [[ ! -f "$CARD" ]]; then
  die "Input card not found: $CARD"
fi
if [[ ! -x "$PR_SH" ]]; then
  die "pr.sh not executable or not found at: $PR_SH"
fi
if [[ ! -x "$CODEXW_SH" ]]; then
  die "codexw.sh not executable or not found at: $CODEXW_SH"
fi
if [[ -z "$PATHS" ]]; then
  die "Missing required --paths"
fi
if [[ "$PATHS" == "." ]]; then
  die "Refusing --paths '.'; pass explicit repo subpaths"
fi
case "$MODE" in
  full-auto|auto-edit|suggest|help) ;;
  *) die "Invalid --mode: $MODE (expected full-auto|auto-edit|suggest|help)" ;;
esac

IFS=',' read -r -a path_arr <<< "$PATHS"
if [[ ${#path_arr[@]} -eq 0 ]]; then
  die "--paths resolved to empty"
fi
for p in "${path_arr[@]}"; do
  p_trim="$(echo "$p" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"
  if [[ -z "$p_trim" ]]; then
    die "--paths contains an empty segment"
  fi
  if [[ "$p_trim" == "." ]]; then
    die "Refusing --paths with '.' segment"
  fi
  if [[ "$p_trim" == ".adl/cards" || "$p_trim" == ".adl/cards/"* ]]; then
    die "Refusing --paths that include .adl/cards"
  fi
done

mkdir -p .adl/logs .adl/cards
codex_cli_preflight

issue_padded="$(issue_from_input_path "$CARD")"
version="$(version_from_card "$CARD")"
if [[ -z "$version" ]]; then
  version="v0.3"
fi
issue=$((10#$issue_padded))
CARD="$(resolve_input_card_path "$issue" "$version")"
out_card="$(resolve_output_card_path "$issue" "$version")"

if [[ ! -f "$CARD" ]]; then
  die "Canonical input card not found: $CARD"
fi

title="$(awk -F': ' '/^Title:/ {print $2; exit}' "$CARD" || true)"
if [[ -z "$title" ]]; then
  title="[${version}] Issue #${issue}"
fi

if [[ -z "$SLUG" ]]; then
  SLUG="$(echo "$title" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//' \
    | cut -c1-60)"
fi

printf '%s\n' "• Issue:   #$issue"
printf '%s\n' "• Version: $version"
printf '%s\n' "• Title:   $title"
printf '%s\n' "• Card:    $CARD"
printf '%s\n' "• Output:  $out_card"
printf '%s\n' "• Mode:    $MODE"
printf '%s\n' "• Slug:    $SLUG"
printf '%s\n' "• Paths:   $PATHS"
printf '\n'

if [[ "$MODE" == "help" ]]; then
  printf '%s\n' "• Help mode: input validation succeeded; no codex/pr actions executed."
  exit 0
fi

"$PR_SH" start "$issue" --slug "$SLUG"

logfile=".adl/logs/${issue}/codex.log"
mkdir -p "$(dirname "$logfile")"

printf '%s\n' "• Running codexw (non-interactive)..."
set +e
"$CODEXW_SH" "$CARD" --mode "$MODE" --log "$logfile"
rc=$?
set -e
if [[ $rc -ne 0 ]]; then
  printf '%s\n' "ERROR: codexw failed (rc=$rc)."
  if working_tree_dirty; then
    printf '%s\n' "ERROR: working tree is dirty after failure."
    git status --short || true
    printf '%s\n' "Next steps: commit, stash, or reset your local changes before retrying."
  fi
  exit "$rc"
fi

printf '\n'
printf '%s\n' "• Codex run complete. Finishing via pr.sh..."

pr_body=$'## ADL Workflow\n- Implemented from the paired ADL input/output card flow.\n- Scope kept minimal and issue-specific.\n\n## Validation\n- cargo fmt\n- cargo clippy --all-targets -- -D warnings\n- cargo test'

"$PR_SH" finish "$issue" \
  --title "$title" \
  --paths "$PATHS" \
  -f "$CARD" \
  --output-card "$out_card" \
  --body "$pr_body"
