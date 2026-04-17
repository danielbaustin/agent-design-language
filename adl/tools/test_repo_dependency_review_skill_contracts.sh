#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/repo-dependency-review/SKILL.md" ]]
[[ -f "${skills_root}/repo-dependency-review/adl-skill.yaml" ]]
[[ -f "${skills_root}/repo-dependency-review/agents/openai.yaml" ]]
[[ -f "${skills_root}/repo-dependency-review/references/dependency-playbook.md" ]]
[[ -f "${skills_root}/repo-dependency-review/references/output-contract.md" ]]
[[ -x "${skills_root}/repo-dependency-review/scripts/prepare_dependency_review.py" ]]
[[ -f "${skills_root}/docs/REPO_DEPENDENCY_REVIEW_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "repo-dependency-review"' "${skills_root}/repo-dependency-review/adl-skill.yaml"
grep -Fq 'id: "repo_dependency_review.v1"' "${skills_root}/repo-dependency-review/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_DEPENDENCY_REVIEW_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-dependency-review/adl-skill.yaml"
grep -Fq "review_packet_mode_requires_target.review_packet_path" "${skills_root}/repo-dependency-review/adl-skill.yaml"
grep -Fq "findings-first" "${skills_root}/repo-dependency-review/SKILL.md"
grep -Fq "scripts/prepare_dependency_review.py" "${skills_root}/repo-dependency-review/SKILL.md"
grep -Fq "Do not install, upgrade, downgrade, pin, unpin, vendor, or remove dependencies" "${skills_root}/repo-dependency-review/references/output-contract.md"
grep -Fq "Schema id: \`repo_dependency_review.v1\`" "${skills_root}/docs/REPO_DEPENDENCY_REVIEW_SKILL_INPUT_SCHEMA.md"
grep -Fq "repo-dependency-review" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
scaffold_root="${tmpdir}/dependency"
mkdir -p "${packet_root}"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "dependency-contract-test",
  "repo_name": "example-repo",
  "publication_allowed": false
}
JSON
cat >"${packet_root}/evidence_index.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.evidence.v1",
  "evidence": [
    {
      "path": "Cargo.toml",
      "category": "manifest",
      "line_count": 42,
      "reason": "Rust dependency manifest",
      "specialist_lanes": ["dependency", "code", "security", "synthesis"]
    },
    {
      "path": "Cargo.lock",
      "category": "lockfile",
      "line_count": 220,
      "reason": "Rust lockfile",
      "specialist_lanes": ["dependency", "synthesis"]
    },
    {
      "path": ".github/workflows/ci.yml",
      "category": "ci",
      "line_count": 88,
      "reason": "CI dependency bootstrap and cache setup",
      "specialist_lanes": ["dependency", "tests", "synthesis"]
    },
    {
      "path": "Dockerfile",
      "category": "docker",
      "line_count": 39,
      "reason": "runtime image dependency setup",
      "specialist_lanes": ["dependency", "security", "synthesis"]
    },
    {
      "path": "NOTICE",
      "category": "docs",
      "line_count": 12,
      "reason": "license attribution surface",
      "specialist_lanes": ["dependency", "docs", "synthesis"]
    },
    {
      "path": "README.md",
      "category": "docs",
      "line_count": 20,
      "reason": "onboarding surface",
      "specialist_lanes": ["docs", "synthesis"]
    }
  ]
}
JSON

python3 "${skills_root}/repo-dependency-review/scripts/prepare_dependency_review.py" \
  "${packet_root}" --out "${scaffold_root}" >/tmp/repo-dependency-review.out
[[ -f "${scaffold_root}/dependency_review_scaffold.json" ]]
[[ -f "${scaffold_root}/dependency_review_scaffold.md" ]]
grep -Fq '"schema": "codebuddy.repo_dependency_review.scaffold.v1"' "${scaffold_root}/dependency_review_scaffold.json"
grep -Fq '"repo_name": "example-repo"' "${scaffold_root}/dependency_review_scaffold.json"
grep -Fq 'Cargo.toml' "${scaffold_root}/dependency_review_scaffold.json"
grep -Fq 'Cargo.lock' "${scaffold_root}/dependency_review_scaffold.md"
grep -Fq 'Dependency Surface Map' "${scaffold_root}/dependency_review_scaffold.md"
grep -Fq 'Candidate Supply-Chain Findings' "${scaffold_root}/dependency_review_scaffold.md"
grep -Fq 'Candidate Dependency Test Gaps' "${scaffold_root}/dependency_review_scaffold.md"
grep -Fq 'Candidate License Review Notes' "${scaffold_root}/dependency_review_scaffold.md"
if grep -R "${tmpdir}" "${scaffold_root}" >/dev/null; then
  echo "dependency review scaffold should not leak absolute temp paths" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/repo-dependency-review/SKILL.md"

echo "PASS test_repo_dependency_review_skill_contracts"
