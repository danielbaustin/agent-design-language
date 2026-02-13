#!/usr/bin/env bash
set -euo pipefail

PR_SH="${PR_SH:-swarm/tools/pr.sh}"
CARD_PATHS_LIB="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/card_paths.sh"
CARD=""
MODE="full-auto"   # full-auto | auto-edit | suggest
SLUG=""
PATHS=""

usage() {
  cat <<'USAGE' >&2
Usage:
  swarm/tools/codex_pr.sh <input-card> --paths "<p1,p2,...>" [--mode full-auto|auto-edit|suggest] [--slug <slug>] [--pr-sh <path>]

Notes:
- --paths is required.
- --paths '.' is forbidden.
- .adl/cards must not be included in --paths.
USAGE
}

die() { echo "ERROR: $*" >&2; exit 2; }

# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

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
if [[ -z "$PATHS" ]]; then
  die "Missing required --paths"
fi
if [[ "$PATHS" == "." ]]; then
  die "Refusing --paths '.'; pass explicit repo subpaths"
fi

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

echo "• Issue:   #$issue"
echo "• Version: $version"
echo "• Title:   $title"
echo "• Card:    $CARD"
echo "• Output:  $out_card"
echo "• Mode:    $MODE"
echo "• Slug:    $SLUG"
echo "• Paths:   $PATHS"
echo

"$PR_SH" start "$issue" --slug "$SLUG"

prompt="$(cat "$CARD")

## Execution notes
- Repository: agent-design-language
- Follow ADL card instructions exactly.
- Work only on the current git branch.
- Make only issue-scoped, minimal changes.
- Run these checks before finishing:
  - cargo fmt
  - cargo clippy --all-targets -- -D warnings
  - cargo test
- Write/update output card at: $out_card
"

stamp="$(date +%Y%m%d-%H%M%S)"
logfile=".adl/logs/issue-${issue}.${stamp}.log"

echo "• Running Codex (non-interactive)..."
if [[ "$MODE" == "full-auto" ]]; then
  codex exec --full-auto "$prompt" | tee "$logfile"
else
  codex exec --approval-mode "$MODE" "$prompt" | tee "$logfile"
fi

echo
echo "• Codex run complete. Finishing via pr.sh..."

pr_body=$'## ADL Workflow\n- Implemented from the paired ADL input/output card flow.\n- Scope kept minimal and issue-specific.\n\n## Validation\n- cargo fmt\n- cargo clippy --all-targets -- -D warnings\n- cargo test'

"$PR_SH" finish "$issue" \
  --title "$title" \
  --paths "$PATHS" \
  -f "$CARD" \
  --output-card "$out_card" \
  --body "$pr_body"
