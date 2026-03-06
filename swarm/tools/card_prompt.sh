#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD_PATHS_LIB="$ROOT_DIR/swarm/tools/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

die() {
  echo "ERROR: $*" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage:
  swarm/tools/card_prompt.sh --issue <number> [--out <path>]
  swarm/tools/card_prompt.sh --input <path> [--out <path>]

Generates a deterministic execution prompt from a structured ADL input card.
USAGE
}

section_body() {
  local file="$1"
  local header="$2"
  awk -v hdr="## ${header}" '
    $0 == hdr { in_section=1; next }
    in_section && /^## / { exit }
    in_section { print }
  ' "$file"
}

meta_field() {
  local file="$1"
  local key="$2"
  awk -F': *' -v k="$key" '$1 == k { sub(/^ +/, "", $2); print $2; exit }' "$file"
}

trim_blank_edges() {
  awk '
    { lines[NR]=$0 }
    END {
      first=1
      while (first<=NR && lines[first] ~ /^[[:space:]]*$/) first++
      last=NR
      while (last>=first && lines[last] ~ /^[[:space:]]*$/) last--
      for (i=first; i<=last; i++) print lines[i]
    }
  '
}

contains_disallowed_content() {
  local file="$1"
  rg -n \
    -e '(^|[^A-Za-z])(\/Users\/|\/home\/|[A-Za-z]:\\)' \
    -e 'AKIA[0-9A-Z]{16}' \
    -e 'gh[pousr]_[A-Za-z0-9_]+' \
    -e 'sk-[A-Za-z0-9]+' \
    "$file" >/dev/null 2>&1
}

display_card_ref() {
  local p="$1"
  local normalized
  normalized="$(echo "$p" | sed 's#\\#/#g')"
  if [[ "$normalized" =~ (.*/)?(\.adl/cards/[0-9]+/input_[0-9]+\.md)$ ]]; then
    echo "${BASH_REMATCH[2]}"
    return 0
  fi
  if [[ "$normalized" =~ (^|/)issue-([0-9]+)__input__v[0-9.]+\.md$ ]]; then
    echo "issue-${BASH_REMATCH[2]} (legacy input card)"
    return 0
  fi
  echo "$(basename "$p")"
}

ISSUE=""
INPUT=""
OUT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --issue) ISSUE="$2"; shift 2 ;;
    --input) INPUT="$2"; shift 2 ;;
    --out) OUT="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) die "unknown argument: $1" ;;
  esac
done

if [[ -n "$ISSUE" && -n "$INPUT" ]]; then
  die "use either --issue or --input, not both"
fi
if [[ -z "$ISSUE" && -z "$INPUT" ]]; then
  die "missing --issue or --input"
fi

if [[ -n "$ISSUE" ]]; then
  ISSUE="$(card_issue_normalize "$ISSUE")"
  INPUT="$(card_input_path "$ISSUE")"
fi

[[ -f "$INPUT" ]] || die "input card not found: $INPUT"

if contains_disallowed_content "$INPUT"; then
  die "input card contains disallowed secret-like token or absolute host path"
fi

task_id="$(meta_field "$INPUT" "Task ID")"
run_id="$(meta_field "$INPUT" "Run ID")"
version="$(meta_field "$INPUT" "Version")"
title="$(meta_field "$INPUT" "Title")"
branch="$(meta_field "$INPUT" "Branch")"

goal="$(section_body "$INPUT" "Goal" | trim_blank_edges)"
acceptance="$(section_body "$INPUT" "Acceptance Criteria" | trim_blank_edges)"
inputs="$(section_body "$INPUT" "Inputs" | trim_blank_edges)"
constraints="$(section_body "$INPUT" "Constraints / Policies" | trim_blank_edges)"
invariants="$(section_body "$INPUT" "System Invariants (must remain true)" | trim_blank_edges)"
checklist="$(section_body "$INPUT" "Reviewer Checklist (machine-readable hints)" | trim_blank_edges)"
non_goals="$(section_body "$INPUT" "Non-goals / Out of scope" | trim_blank_edges)"
risks="$(section_body "$INPUT" "Notes / Risks" | trim_blank_edges)"
input_ref="$(display_card_ref "$INPUT")"

render_or_na() {
  local value="$1"
  if [[ -n "$value" ]]; then
    printf '%s\n' "$value"
  else
    echo "(not provided)"
  fi
}

prompt="$(
  cat <<EOF
Work Prompt — ${task_id:-issue-unknown}

Context
- Task ID: ${task_id:-unknown}
- Run ID: ${run_id:-unknown}
- Version: ${version:-unknown}
- Title: ${title:-unknown}
- Branch: ${branch:-unknown}
- Input Card: ${input_ref}

Goal
$(render_or_na "$goal")

Acceptance Criteria
$(render_or_na "$acceptance")

Inputs
$(render_or_na "$inputs")

Constraints / Policies
$(render_or_na "$constraints")

System Invariants (must remain true)
$(render_or_na "$invariants")

Reviewer Checklist (machine-readable hints)
$(render_or_na "$checklist")

Non-goals / Out of scope
$(render_or_na "$non_goals")

Notes / Risks
$(render_or_na "$risks")

Execution Rules
- Keep changes deterministic and replay-safe.
- Do not broaden scope beyond the card.
- Run validation commands required by the card/repo.
- Update the paired output card with concrete evidence.
EOF
)"

if [[ -n "$OUT" ]]; then
  mkdir -p "$(dirname "$OUT")"
  printf '%s\n' "$prompt" > "$OUT"
else
  printf '%s\n' "$prompt"
fi
