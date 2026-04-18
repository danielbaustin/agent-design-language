#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/architecture-fitness-function-author/SKILL.md" ]]
[[ -f "${skills_root}/architecture-fitness-function-author/adl-skill.yaml" ]]
[[ -f "${skills_root}/architecture-fitness-function-author/agents/openai.yaml" ]]
[[ -f "${skills_root}/architecture-fitness-function-author/references/fitness-function-playbook.md" ]]
[[ -f "${skills_root}/architecture-fitness-function-author/references/output-contract.md" ]]
[[ -x "${skills_root}/architecture-fitness-function-author/scripts/author_architecture_fitness_functions.py" ]]
[[ -f "${skills_root}/docs/ARCHITECTURE_FITNESS_FUNCTION_AUTHOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "architecture-fitness-function-author"' "${skills_root}/architecture-fitness-function-author/adl-skill.yaml"
grep -Fq 'id: "architecture_fitness_function_author.v1"' "${skills_root}/architecture-fitness-function-author/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ARCHITECTURE_FITNESS_FUNCTION_AUTHOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/architecture-fitness-function-author/adl-skill.yaml"
grep -Fq "author_from_review_packet_requires_target.review_packet_path" "${skills_root}/architecture-fitness-function-author/adl-skill.yaml"
grep -Fq "turns recurring architecture risks into bounded checks" "${skills_root}/architecture-fitness-function-author/SKILL.md"
grep -Fq "scripts/author_architecture_fitness_functions.py" "${skills_root}/architecture-fitness-function-author/SKILL.md"
grep -Fq "Do not edit tests" "${skills_root}/architecture-fitness-function-author/references/output-contract.md"
grep -Fq "Schema id: \`architecture_fitness_function_author.v1\`" "${skills_root}/docs/ARCHITECTURE_FITNESS_FUNCTION_AUTHOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "architecture-fitness-function-author" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
plan_root="${tmpdir}/fitness-functions"
mkdir -p "${packet_root}"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "architecture-fitness-function-author-contract-test",
  "repo_name": "example-repo",
  "publication_allowed": false
}
JSON
cat >"${packet_root}/evidence_index.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.evidence.v1",
  "evidence": [
    {
      "path": "adl/src/execute/state.rs",
      "category": "code",
      "line_count": 220,
      "reason": "runtime state lifecycle contract should remain stable",
      "specialist_lanes": ["architecture", "tests", "synthesis"]
    },
    {
      "path": "Cargo.toml",
      "category": "manifest",
      "line_count": 80,
      "reason": "dependency boundary and package policy surface",
      "specialist_lanes": ["architecture", "dependency", "synthesis"]
    }
  ]
}
JSON
cat >"${packet_root}/architecture-review.md" <<'MD'
# Architecture Review

## Candidate Fitness Functions

- Rule: CLI code must not import runtime-internal state modules directly.
  Source: adl/src/execute/state.rs
  Rationale: Preserve runtime lifecycle ownership and dependency direction.

- Rule: The provider boundary decision needs an ADR before automation.
  Source: docs/architecture/provider-boundary.md
  Rationale: The tradeoff is still human judgment.
MD

python3 "${skills_root}/architecture-fitness-function-author/scripts/author_architecture_fitness_functions.py" \
  "${packet_root}" --out "${plan_root}" --max-rules 6 >/tmp/architecture-fitness-function-author.out
[[ -f "${plan_root}/architecture_fitness_functions.json" ]]
[[ -f "${plan_root}/architecture_fitness_functions.md" ]]
grep -Fq '"schema": "codebuddy.architecture_fitness_functions.v1"' "${plan_root}/architecture_fitness_functions.json"
grep -Fq '"repo_name": "example-repo"' "${plan_root}/architecture_fitness_functions.json"
grep -Fq '"classification": "machine_checkable"' "${plan_root}/architecture_fitness_functions.json"
grep -Fq '"classification": "human_judgment"' "${plan_root}/architecture_fitness_functions.json"
grep -Fq 'Machine-Checkable Invariants' "${plan_root}/architecture_fitness_functions.md"
grep -Fq 'Human-Judgment Candidates' "${plan_root}/architecture_fitness_functions.md"
grep -Fq 'Expected Failure Modes' "${plan_root}/architecture_fitness_functions.md"
grep -Fq 'Implementation Handoffs' "${plan_root}/architecture_fitness_functions.md"
if grep -R "${tmpdir}" "${plan_root}" >/dev/null; then
  echo "architecture fitness-function plan should not leak absolute temp paths" >&2
  exit 1
fi
if find "${plan_root}" -name '*.rs' -o -name '*.sh' -o -name '*.yml' | grep -q .; then
  echo "architecture fitness-function author should not write implementation files" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/architecture-fitness-function-author/SKILL.md"

echo "PASS test_architecture_fitness_function_author_skill_contracts"
