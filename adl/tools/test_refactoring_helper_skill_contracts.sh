#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/refactoring-helper/SKILL.md" ]]
[[ -f "${skills_root}/refactoring-helper/adl-skill.yaml" ]]
[[ -f "${skills_root}/refactoring-helper/agents/openai.yaml" ]]
[[ -f "${skills_root}/refactoring-helper/references/refactoring-playbook.md" ]]
[[ -f "${skills_root}/refactoring-helper/references/output-contract.md" ]]
[[ -x "${skills_root}/refactoring-helper/scripts/plan_refactor.py" ]]
[[ -f "${skills_root}/docs/REFACTORING_HELPER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "refactoring-helper"' "${skills_root}/refactoring-helper/adl-skill.yaml"
grep -Fq 'id: "refactoring_helper.v1"' "${skills_root}/refactoring-helper/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REFACTORING_HELPER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/refactoring-helper/adl-skill.yaml"
grep -Fq "bounded_target_must_be_explicit" "${skills_root}/refactoring-helper/adl-skill.yaml"
grep -Fq "behavior-preserving refactor plan" "${skills_root}/refactoring-helper/SKILL.md"
grep -Fq "Do not claim implementation" "${skills_root}/refactoring-helper/references/output-contract.md"
grep -Fq "Schema id: \`refactoring_helper.v1\`" "${skills_root}/docs/REFACTORING_HELPER_SKILL_INPUT_SCHEMA.md"

refactor_root="${tmpdir}/refactor"
plan_root="${tmpdir}/refactor-plan"
mkdir -p "${refactor_root}"
cat >"${refactor_root}/refactor_manifest.json" <<'JSON'
{
  "run_id": "refactoring-helper-contract-test",
  "scope": "issue-2043",
  "mode": "plan_refactor",
  "target_paths": [
    "adl/src/cli/pr_cmd.rs",
    "adl/src/cli/pr_cmd_validate.rs"
  ]
}
JSON
cat >"${refactor_root}/current_behavior.md" <<'MD'
# Current Behavior

The PR lifecycle keeps root main clean while binding implementation work to issue worktrees.
MD
cat >"${refactor_root}/refactor_intent.md" <<'MD'
# Refactor Intent

Improve internal structure while preserving externally visible behavior.
MD
cat >"${refactor_root}/invariants.md" <<'MD'
# Invariants

## Invariants

- Root main remains clean and is not used for tracked implementation edits.
- Issue worktrees remain traceable to the issue branch.
MD
cat >"${refactor_root}/validation.md" <<'MD'
# Validation

## Validation Commands

- cargo test -p adl --lib cli::pr_cmd
- bash adl/tools/test_pr_lifecycle_contracts.sh
MD
cat >"${refactor_root}/known_risks.md" <<'MD'
# Known Risks

## Risks

- Lifecycle helper refactors can accidentally weaken root checkout guardrails.
MD

python3 "${skills_root}/refactoring-helper/scripts/plan_refactor.py" \
  "${refactor_root}" --out "${plan_root}" --max-slices 2 >/tmp/refactoring-helper.out
[[ -f "${plan_root}/refactor_plan.json" ]]
[[ -f "${plan_root}/refactor_plan.md" ]]
grep -Fq '"schema": "adl.refactor_plan.v1"' "${plan_root}/refactor_plan.json"
grep -Fq '"run_id": "refactoring-helper-contract-test"' "${plan_root}/refactor_plan.json"
grep -Fq '"status": "ready"' "${plan_root}/refactor_plan.json"
grep -Fq '"performed_refactor": false' "${plan_root}/refactor_plan.json"
grep -Fq '"mutated_repository": false' "${plan_root}/refactor_plan.json"
grep -Fq "## Refactor Plan Summary" "${plan_root}/refactor_plan.md"
grep -Fq "## Refactor Slices" "${plan_root}/refactor_plan.md"
grep -Fq "Performed refactor: false." "${plan_root}/refactor_plan.md"
grep -Fq "Mutated repository: false." "${plan_root}/refactor_plan.md"

missing_root="${tmpdir}/missing-target"
mkdir -p "${missing_root}"
python3 "${skills_root}/refactoring-helper/scripts/plan_refactor.py" \
  "${missing_root}" --out "${tmpdir}/missing-plan" >/tmp/refactoring-helper-missing.out
grep -Fq '"status": "not_run"' "${tmpdir}/missing-plan/refactor_plan.json"

if grep -R "${tmpdir}" "${plan_root}" "${tmpdir}/missing-plan" >/dev/null; then
  echo "refactoring helper artifacts should not leak absolute temp paths" >&2
  exit 1
fi
if find "${plan_root}" -name '*.rs' -o -name '*.py' -o -name '*.ts' | grep -q .; then
  echo "refactoring helper should not write implementation files" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/refactoring-helper/SKILL.md"

echo "PASS test_refactoring_helper_skill_contracts"
