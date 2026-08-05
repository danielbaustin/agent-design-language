#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
editor="/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-edit"

apply_edit() {
  local issue="$1" card="$2" reason="$3" operation="$4"
  local index="$root/.csdlc/issues/$issue/index.json"
  local request="$root/.csdlc/prepared/issues/$issue/edit-request.json"
  jq -n \
    --argjson issue "$issue" \
    --arg card "$card" \
    --arg claim_id "claim-$issue-v092-preparation" \
    --arg reason "$reason" \
    --argjson generation "$(jq '.generation' "$index")" \
    --arg digest "$(jq -r '.digest' "$index")" \
    --argjson operation "$operation" \
    '{issue:$issue,expected_generation:$generation,expected_digest:$digest,claim_id:$claim_id,actor:"codex:5854-card-editor",reason:$reason,card:$card,operation:$operation}' > "$request"
  printf 'apply issue=%s card=%s reason=%s\n' "$issue" "$card" "$reason"
  "$editor" --repo "$root" apply --request "$request" >/dev/null
}

for issue in ${ISSUES:-5835 5836 5838 5839 5840 5844 5845}; do
  spec="$root/.csdlc/prepared/issues/$issue/readiness-spec.json"

  if [[ "$issue" != "${RESUME_ACCEPTANCE_ISSUE:-}" ]]; then
    for entry in \
      "sip:goal:goal" \
      "sip:required_outcome:required_outcome" \
      "stp:task_boundary:task_boundary" \
      "spp:plan_summary:plan_summary" \
      "srp:review_scope:review_scope"; do
      IFS=: read -r card field key <<<"$entry"
      value="$(jq -r --arg key "$key" '.[$key]' "$spec")"
      apply_edit "$issue" "$card" "Set issue-specific $field from the reviewed readiness spec." \
        "$(jq -nc --arg field "$field" --arg value "$value" '{operation:"replan",field:$field,value:$value}')"
    done

    apply_edit "$issue" sip "Replace operator constraints from the reviewed readiness spec." \
      "$(jq -c '{operation:"replace_operator_constraints",values:.operator_constraints}' "$spec")"

    for card in sip stp spp; do
      jq -r --arg card "$card" '.collections[$card] | keys[]' "$spec" | while read -r field; do
        apply_edit "$issue" "$card" "Replace $field from the reviewed readiness spec." \
          "$(jq -c --arg card "$card" --arg field "$field" '{operation:"replace_planning_collection",field:$field,values:.collections[$card][$field]}' "$spec")"
      done
    done
  fi

  apply_edit "$issue" spp "Replace acceptance, plan, and validation DAG from the reviewed readiness spec." \
    "$(jq -c '{operation:"replace_acceptance_plan",acceptance_criteria:.acceptance_criteria,steps:.steps,validation_lanes:.validation_lanes}' "$spec")"

  apply_edit "$issue" srp "Replace pre-execution review prompts from the reviewed readiness spec." \
    "$(jq -c '{operation:"replace_planning_collection",field:"review_prompts",values:.review_prompts}' "$spec")"
done

echo "typed readiness specs applied"
