#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/review-to-test-planner/SKILL.md" ]]
[[ -f "${skills_root}/review-to-test-planner/adl-skill.yaml" ]]
[[ -f "${skills_root}/review-to-test-planner/agents/openai.yaml" ]]
[[ -f "${skills_root}/review-to-test-planner/references/test-planning-playbook.md" ]]
[[ -f "${skills_root}/review-to-test-planner/references/output-contract.md" ]]
[[ -x "${skills_root}/review-to-test-planner/scripts/plan_review_tests.py" ]]
[[ -f "${skills_root}/docs/REVIEW_TO_TEST_PLANNER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "review-to-test-planner"' "${skills_root}/review-to-test-planner/adl-skill.yaml"
grep -Fq 'id: "review_to_test_planner.v1"' "${skills_root}/review-to-test-planner/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REVIEW_TO_TEST_PLANNER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/review-to-test-planner/adl-skill.yaml"
grep -Fq "plan_from_review_packet_requires_target.review_packet_path" "${skills_root}/review-to-test-planner/adl-skill.yaml"
grep -Fq "between review artifacts and \`test-generator\`" "${skills_root}/review-to-test-planner/SKILL.md"
grep -Fq "scripts/plan_review_tests.py" "${skills_root}/review-to-test-planner/SKILL.md"
grep -Fq "Do not write tests" "${skills_root}/review-to-test-planner/references/output-contract.md"
grep -Fq "Schema id: \`review_to_test_planner.v1\`" "${skills_root}/docs/REVIEW_TO_TEST_PLANNER_SKILL_INPUT_SCHEMA.md"
grep -Fq "review-to-test-planner" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

review_root="${tmpdir}/review-packet"
plan_root="${tmpdir}/review-to-test-plan"
mkdir -p "${review_root}"
cat >"${review_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "review-to-test-planner-contract-test",
  "repo_name": "example-repo",
  "publication_allowed": false
}
JSON
cat >"${review_root}/evidence_index.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.evidence.v1",
  "evidence": [
    {
      "path": "adl/src/execute/state.rs",
      "category": "code",
      "line_count": 220,
      "reason": "runtime state lifecycle implementation with pause and resume behavior",
      "specialist_lanes": ["code", "tests", "synthesis"]
    },
    {
      "path": "adl/tools/redact.sh",
      "category": "tooling",
      "line_count": 40,
      "reason": "redaction helper protects synthetic secret output",
      "specialist_lanes": ["security", "tests", "synthesis"]
    }
  ]
}
JSON
cat >"${review_root}/synthesis.md" <<'MD'
# Review

## Finding 1

[P1] Resume skips stale runtime state
File: adl/src/execute/state.rs
Scenario: A paused run resumes after the state file changes.
Impact: The runtime can report success without exercising the recovery path.
Evidence: The state lifecycle implementation lacks a direct regression proof.

## Finding 2

[P2] Real credential redaction must not use production secrets
File: adl/tools/redact.sh
Scenario: Tests need secret-like inputs.
Impact: Unsafe test generation could expose credentials if it uses real data.
Evidence: Use synthetic secret-like values only.
MD

python3 "${skills_root}/review-to-test-planner/scripts/plan_review_tests.py" \
  "${review_root}" --out "${plan_root}" --max-tasks 4 >/tmp/review-to-test-planner.out
[[ -f "${plan_root}/review_to_test_plan.json" ]]
[[ -f "${plan_root}/review_to_test_plan.md" ]]
grep -Fq '"schema": "codebuddy.review_to_test_plan.v1"' "${plan_root}/review_to_test_plan.json"
grep -Fq '"repo_name": "example-repo"' "${plan_root}/review_to_test_plan.json"
grep -Fq '"generation_status": "recommended"' "${plan_root}/review_to_test_plan.json"
grep -Fq '"generation_status": "unsafe"' "${plan_root}/review_to_test_plan.json"
grep -Fq '"skill_input_schema": "test_generator.v1"' "${plan_root}/review_to_test_plan.json"
grep -Fq 'Findings To Test Map' "${plan_root}/review_to_test_plan.md"
grep -Fq 'Generation Status Summary' "${plan_root}/review_to_test_plan.md"
grep -Fq 'Test Generator Handoffs' "${plan_root}/review_to_test_plan.md"
grep -Fq 'Deferred And Unsafe Tasks' "${plan_root}/review_to_test_plan.md"
if grep -R "${tmpdir}" "${plan_root}" >/dev/null; then
  echo "review-to-test plan should not leak absolute temp paths" >&2
  exit 1
fi
if find "${plan_root}" -name '*_test.rs' -o -name 'test_*.py' -o -name '*.test.ts' | grep -q .; then
  echo "review-to-test planner should not write test files" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/review-to-test-planner/SKILL.md"

echo "PASS test_review_to_test_planner_skill_contracts"
