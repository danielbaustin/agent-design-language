#!/usr/bin/env bash
set -euo pipefail

root="${1:-.}"
packet="$root/.csdlc/prepared/issues/5613"

test -f "$root/docs/templates/prompts/current.json"
test -f "$packet/design.md"
test -f "$packet/diagram.mmd"
test -f "$packet/bootstrap-request.json"
test -f "$packet/bind-request.json"
test -f "$packet/validate_scope.sh"

jq -e '.csdlc_prompt_template_set == "1.0.3" and .generations.csdlc_v2_native.template_set == "1.0.0"' \
  "$root/docs/templates/prompts/current.json" >/dev/null
jq -e '
  .issue == 5613 and
  .repository == "danielbaustin/agent-design-language" and
  .claim.branch == "codex/5613-terminal-projection-repair" and
  (.claim.protected_paths | length >= 10) and
  (.initial.acceptance_criteria | length == 10) and
  (.initial.validation_lanes | length == 4) and
  (.initial.operator_constraints | index("No raw gh, AWS, Runtime v2, or ADL-v2 product changes"))
' "$packet/bootstrap-request.json" >/dev/null

for issue in 5337 5339 5358 5591 5602; do
  receipt="$(git -C "$root" rev-parse --git-common-dir)/csdlc-v2/closeout/$issue.json"
  test -f "$receipt"
  jq -e --argjson issue "$issue" '.issue == $issue and .record.phase == "closed_out" and .record.claim == null' "$receipt" >/dev/null
done

git -C "$root" cat-file -e 461713dc10d26fa5336a054c07ef1844f804ec8f^{commit}
git -C "$root" cat-file -e 817126889942fc57820bf9f05f5cc40e2debd683^{commit}
git -C "$root" cat-file -e 8cfb7b25ad246dd411a57ecc4fda8e47665912fc^{commit}

if git -C "$root" diff --name-only origin/main...HEAD | rg -q '^(adl-runtime|adl-runtime-kernel|adl-v2)/'; then
  echo "forbidden product scope in preparation branch" >&2
  exit 1
fi

bash "$packet/validate_scope.sh" "$root"

printf 'issue 5613 preparation contract: pass\n'
