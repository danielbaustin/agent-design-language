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
  adl/tools/lint_prompt_spec.sh --issue <number>
  adl/tools/lint_prompt_spec.sh --input <path>
USAGE
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

contains_disallowed_content() {
  local file="$1"
  rg -n \
    -e '(^|[^A-Za-z])(\/Users\/|\/home\/|[A-Za-z]:\\)' \
    -e 'AKIA[0-9A-Z]{16}' \
    -e 'gh[pousr]_[A-Za-z0-9_]+' \
    -e '(^|[^A-Za-z0-9])sk-[A-Za-z0-9_]{8,}($|[^A-Za-z0-9_])' \
    "$file" >/dev/null 2>&1
}

ISSUE=""
INPUT=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --issue) ISSUE="$2"; shift 2 ;;
    --input) INPUT="$2"; shift 2 ;;
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

spec="$(prompt_spec_yaml "$INPUT")"
[[ -n "$spec" ]] || die "missing Prompt Spec YAML block"

required_top_keys=(
  prompt_schema
  actor
  model
  inputs
  outputs
  constraints
  automation_hints
  review_surfaces
)

for key in "${required_top_keys[@]}"; do
  if ! awk -v k="$key" '$0 ~ ("^" k ":[[:space:]]*$") || $0 ~ ("^" k ":[[:space:]]+.+$") { found=1 } END { exit(found ? 0 : 1) }' <<<"$spec"; then
    die "Prompt Spec missing required key: $key"
  fi
done

schema="$(awk -F': *' '$1=="prompt_schema" {print $2; exit}' <<<"$spec")"
[[ "$schema" == "adl.v1" ]] || die "unsupported prompt_schema: ${schema:-<empty>}"

if ! awk '/^inputs:[[:space:]]*$/ { in_inputs=1; next } in_inputs && /^  sections:[[:space:]]*$/ { found=1; exit } END { exit(found ? 0 : 1) }' <<<"$spec"; then
  die "Prompt Spec missing inputs.sections"
fi

section_ids=()
while IFS= read -r line; do
  section_ids+=("$line")
done < <(
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
  ' <<<"$spec"
)

[[ "${#section_ids[@]}" -gt 0 ]] || die "inputs.sections must include at least one section id"

supported_sections=(
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

for id in "${section_ids[@]}"; do
  ok=0
  for supported in "${supported_sections[@]}"; do
    if [[ "$id" == "$supported" ]]; then
      ok=1
      break
    fi
  done
  [[ "$ok" -eq 1 ]] || die "unsupported inputs.sections entry: $id"
done

for bool_key in include_system_invariants include_reviewer_checklist disallow_secrets disallow_absolute_host_paths; do
  value="$(awk -v key="$bool_key" '
    /^[[:space:]]*[A-Za-z0-9_]+:[[:space:]]*/ {
      split($0, parts, ":")
      field=parts[1]
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", field)
      if (field == key) {
        sub(/^[^:]*:[[:space:]]*/, "", $0)
        val=tolower($0)
        gsub(/[[:space:]]+/, "", val)
        print val
        exit
      }
    }
  ' <<<"$spec")"
  if [[ -z "$value" ]]; then
    die "Prompt Spec missing constraints.${bool_key}"
  fi
  if [[ "$value" != "true" && "$value" != "false" ]]; then
    die "Prompt Spec constraints.${bool_key} must be true|false (found: $value)"
  fi
done

for bool_key in source_issue_prompt_required target_files_surfaces_recommended validation_plan_required required_outcome_type_supported; do
  value="$(awk -v key="$bool_key" '
    /^[[:space:]]*[A-Za-z0-9_]+:[[:space:]]*/ {
      split($0, parts, ":")
      field=parts[1]
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", field)
      if (field == key) {
        sub(/^[^:]*:[[:space:]]*/, "", $0)
        val=tolower($0)
        gsub(/[[:space:]]+/, "", val)
        print val
        exit
      }
    }
  ' <<<"$spec")"
  if [[ -z "$value" ]]; then
    die "Prompt Spec missing automation_hints.${bool_key}"
  fi
  if [[ "$value" != "true" && "$value" != "false" ]]; then
    die "Prompt Spec automation_hints.${bool_key} must be true|false (found: $value)"
  fi
done

review_surfaces=()
while IFS= read -r line; do
  review_surfaces+=("$line")
done < <(
  awk '
    /^review_surfaces:[[:space:]]*$/ { in_review=1; next }
    in_review && /^[A-Za-z0-9_]+:[[:space:]]*$/ { in_review=0; next }
    in_review && /^  -[[:space:]]+/ {
      line=$0
      sub(/^[[:space:]]*-[[:space:]]*/, "", line)
      print line
      next
    }
    in_review && !/^  / { in_review=0 }
  ' <<<"$spec"
)

[[ "${#review_surfaces[@]}" -gt 0 ]] || die "review_surfaces must include at least one protocol id"

has_checklist=0
has_output=0
has_reviewer=0
for surface in "${review_surfaces[@]}"; do
  case "$surface" in
    card_review_checklist.v1) has_checklist=1 ;;
    card_review_output.v1) has_output=1 ;;
    card_reviewer_gpt.v1.1) has_reviewer=1 ;;
  esac
done

[[ "$has_checklist" -eq 1 ]] || die "review_surfaces missing required protocol id: card_review_checklist.v1"
[[ "$has_output" -eq 1 ]] || die "review_surfaces missing required protocol id: card_review_output.v1"
[[ "$has_reviewer" -eq 1 ]] || die "review_surfaces missing required protocol id: card_reviewer_gpt.v1.1"

checklist_pos=-1
output_pos=-1
reviewer_pos=-1
for i in "${!review_surfaces[@]}"; do
  case "${review_surfaces[$i]}" in
    card_review_checklist.v1) checklist_pos="$i" ;;
    card_review_output.v1) output_pos="$i" ;;
    card_reviewer_gpt.v1.1) reviewer_pos="$i" ;;
  esac
done

if [[ "$checklist_pos" -gt "$output_pos" || "$output_pos" -gt "$reviewer_pos" ]]; then
  die "review_surfaces must appear in canonical order: checklist -> output -> reviewer"
fi

echo "PASS: Prompt Spec is valid for $INPUT"
