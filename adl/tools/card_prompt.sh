#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD_PATHS_LIB="$ROOT_DIR/adl/tools/card_paths.sh"
# shellcheck disable=SC1090
source "$CARD_PATHS_LIB"

die() {
  echo "ERROR: $*" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage:
  adl/tools/card_prompt.sh --issue <number> [--out <path>]
  adl/tools/card_prompt.sh --input <path> [--out <path>]

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

prompt_spec_yaml() {
  local file="$1"
  awk '
    $0 == "## Prompt Spec" { in_prompt=1; next }
    in_prompt && /^## / { exit }
    in_prompt { print }
  ' "$file" | awk '
    /^```yaml$/ { in_yaml=1; next }
    in_yaml && /^```$/ { exit }
    in_yaml { print }
  '
}

prompt_spec_sections() {
  local yaml="$1"
  awk '
    /^inputs:[[:space:]]*$/ { in_inputs=1; next }
    in_inputs && /^outputs:[[:space:]]*$/ { in_inputs=0; in_sections=0; next }
    in_inputs && /^  sections:[[:space:]]*$/ { in_sections=1; next }
    in_sections && /^    -[[:space:]]+/ {
      line=$0
      sub(/^[[:space:]]*-[[:space:]]*/, "", line)
      print line
      next
    }
    in_sections && !/^    / { in_sections=0 }
  ' <<<"$yaml"
}

prompt_spec_bool() {
  local yaml="$1"
  local key="$2"
  awk -v k="$key" '
    /^[[:space:]]*[A-Za-z0-9_]+:[[:space:]]*/ {
      split($0, parts, ":")
      field=parts[1]
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", field)
      if (field == k) {
        sub(/^[^:]*:[[:space:]]*/, "", $0)
        val=tolower($0)
        gsub(/[[:space:]]+/, "", val)
        if (val == "true" || val == "false") {
          print val
          exit
        }
      }
    }
  ' <<<"$yaml"
}

section_id_to_header() {
  case "$1" in
    goal) echo "Goal" ;;
    required_outcome) echo "Required Outcome" ;;
    acceptance_criteria) echo "Acceptance Criteria" ;;
    inputs) echo "Inputs" ;;
    target_files_surfaces) echo "Target Files / Surfaces" ;;
    validation_plan) echo "Validation Plan" ;;
    demo_proof_requirements) echo "Demo / Proof Requirements" ;;
    constraints_policies) echo "Constraints / Policies" ;;
    system_invariants) echo "System Invariants (must remain true)" ;;
    reviewer_checklist) echo "Reviewer Checklist (machine-readable hints)" ;;
    non_goals_out_of_scope) echo "Non-goals / Out of scope" ;;
    notes_risks) echo "Notes / Risks" ;;
    instructions_to_agent) echo "Instructions to the Agent" ;;
    *) echo "" ;;
  esac
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
  if [[ "$normalized" =~ (.*/)?(\.adl/[[:alnum:]._-]+/tasks/issue-[0-9]+__[^/]+/sip\.md)$ ]]; then
    echo "${BASH_REMATCH[2]}"
    return 0
  fi
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
  INPUT="$(resolve_input_card_path "$ISSUE" "")"
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

prompt_spec="$(prompt_spec_yaml "$INPUT")"

section_ids=()
while IFS= read -r line; do
  section_ids+=("$line")
done < <(prompt_spec_sections "$prompt_spec")
if [[ "${#section_ids[@]}" -eq 0 ]]; then
  section_ids=(
    goal
    required_outcome
    acceptance_criteria
    inputs
    target_files_surfaces
    validation_plan
    demo_proof_requirements
    constraints_policies
    system_invariants
    reviewer_checklist
    non_goals_out_of_scope
    notes_risks
    instructions_to_agent
  )
fi

include_system_invariants="$(prompt_spec_bool "$prompt_spec" "include_system_invariants")"
include_reviewer_checklist="$(prompt_spec_bool "$prompt_spec" "include_reviewer_checklist")"
if [[ -z "$include_system_invariants" ]]; then
  include_system_invariants="true"
fi
if [[ -z "$include_reviewer_checklist" ]]; then
  include_reviewer_checklist="true"
fi

if [[ "$include_system_invariants" == "true" ]]; then
  has_invariants=0
  for id in "${section_ids[@]}"; do
    if [[ "$id" == "system_invariants" ]]; then
      has_invariants=1
      break
    fi
  done
  if [[ "$has_invariants" -eq 0 ]]; then
    section_ids+=("system_invariants")
  fi
fi

if [[ "$include_reviewer_checklist" == "true" ]]; then
  has_checklist=0
  for id in "${section_ids[@]}"; do
    if [[ "$id" == "reviewer_checklist" ]]; then
      has_checklist=1
      break
    fi
  done
  if [[ "$has_checklist" -eq 0 ]]; then
    section_ids+=("reviewer_checklist")
  fi
fi

ordered_section_ids=()
for id in "${section_ids[@]}"; do
  [[ -n "$id" ]] || continue
  seen=0
  for existing in "${ordered_section_ids[@]-}"; do
    if [[ "$existing" == "$id" ]]; then
      seen=1
      break
    fi
  done
  if [[ "$seen" -eq 0 ]]; then
    ordered_section_ids+=("$id")
  fi
done

input_ref="$(display_card_ref "$INPUT")"

render_or_na() {
  local value="$1"
  if [[ -n "$value" ]]; then
    printf '%s\n' "$value"
  else
    echo "(not provided)"
  fi
}

render_section_by_id() {
  local file="$1"
  local id="$2"
  local header
  header="$(section_id_to_header "$id")"
  if [[ -z "$header" ]]; then
    return 0
  fi
  local body
  body="$(section_body "$file" "$header" | trim_blank_edges)"
  cat <<EOF
$header
$(render_or_na "$body")
EOF
}

sections_rendered=""
for id in "${ordered_section_ids[@]-}"; do
  header="$(section_id_to_header "$id")"
  [[ -n "$header" ]] || continue
  if [[ "$id" == "system_invariants" && "$include_system_invariants" != "true" ]]; then
    continue
  fi
  if [[ "$id" == "reviewer_checklist" && "$include_reviewer_checklist" != "true" ]]; then
    continue
  fi
  rendered="$(render_section_by_id "$INPUT" "$id")"
  if [[ -n "$sections_rendered" ]]; then
    sections_rendered+=$'\n\n'
  fi
  sections_rendered+="$rendered"
done

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

${sections_rendered}

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
