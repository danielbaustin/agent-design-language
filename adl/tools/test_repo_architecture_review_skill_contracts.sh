#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/repo-architecture-review/SKILL.md" ]]
[[ -f "${skills_root}/repo-architecture-review/adl-skill.yaml" ]]
[[ -f "${skills_root}/repo-architecture-review/agents/openai.yaml" ]]
[[ -f "${skills_root}/repo-architecture-review/references/architecture-playbook.md" ]]
[[ -f "${skills_root}/repo-architecture-review/references/output-contract.md" ]]
[[ -x "${skills_root}/repo-architecture-review/scripts/prepare_architecture_review.py" ]]
[[ -f "${skills_root}/docs/REPO_ARCHITECTURE_REVIEW_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "repo-architecture-review"' "${skills_root}/repo-architecture-review/adl-skill.yaml"
grep -Fq 'id: "repo_architecture_review.v1"' "${skills_root}/repo-architecture-review/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_ARCHITECTURE_REVIEW_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-architecture-review/adl-skill.yaml"
grep -Fq "review_packet_mode_requires_target.review_packet_path" "${skills_root}/repo-architecture-review/adl-skill.yaml"
grep -Fq "findings-first and source-grounded" "${skills_root}/repo-architecture-review/SKILL.md"
grep -Fq "scripts/prepare_architecture_review.py" "${skills_root}/repo-architecture-review/SKILL.md"
grep -Fq "Do not author diagrams, ADRs, fitness-function code, issues, or synthesis" "${skills_root}/repo-architecture-review/references/output-contract.md"
grep -Fq "Schema id: \`repo_architecture_review.v1\`" "${skills_root}/docs/REPO_ARCHITECTURE_REVIEW_SKILL_INPUT_SCHEMA.md"
grep -Fq "repo-architecture-review" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
scaffold_root="${tmpdir}/architecture"
mkdir -p "${packet_root}"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "architecture-contract-test",
  "repo_name": "example-repo",
  "publication_allowed": false
}
JSON
cat >"${packet_root}/evidence_index.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.evidence.v1",
  "evidence": [
    {
      "path": "docs/architecture/runtime.md",
      "category": "architecture_docs",
      "line_count": 42,
      "reason": "architecture documentation surface",
      "specialist_lanes": ["architecture", "docs", "diagrams", "synthesis"]
    },
    {
      "path": "adl/src/execute/state.rs",
      "category": "code",
      "line_count": 220,
      "reason": "runtime state lifecycle implementation",
      "specialist_lanes": ["architecture", "code", "tests", "synthesis"]
    },
    {
      "path": "README.md",
      "category": "docs",
      "line_count": 12,
      "reason": "onboarding surface",
      "specialist_lanes": ["docs", "synthesis"]
    }
  ]
}
JSON

python3 "${skills_root}/repo-architecture-review/scripts/prepare_architecture_review.py" \
  "${packet_root}" --out "${scaffold_root}" >/tmp/repo-architecture-review.out
[[ -f "${scaffold_root}/architecture_review_scaffold.json" ]]
[[ -f "${scaffold_root}/architecture_review_scaffold.md" ]]
grep -Fq '"schema": "codebuddy.repo_architecture_review.scaffold.v1"' "${scaffold_root}/architecture_review_scaffold.json"
grep -Fq '"repo_name": "example-repo"' "${scaffold_root}/architecture_review_scaffold.json"
grep -Fq 'docs/architecture/runtime.md' "${scaffold_root}/architecture_review_scaffold.json"
grep -Fq 'adl/src/execute/state.rs' "${scaffold_root}/architecture_review_scaffold.md"
grep -Fq 'Candidate Diagram Tasks' "${scaffold_root}/architecture_review_scaffold.md"
grep -Fq 'Candidate ADRs' "${scaffold_root}/architecture_review_scaffold.md"
grep -Fq 'Candidate Fitness Functions' "${scaffold_root}/architecture_review_scaffold.md"
if grep -R "${tmpdir}" "${scaffold_root}" >/dev/null; then
  echo "architecture review scaffold should not leak absolute temp paths" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/repo-architecture-review/SKILL.md"

echo "PASS test_repo_architecture_review_skill_contracts"
