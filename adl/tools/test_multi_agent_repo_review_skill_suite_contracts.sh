#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

specialists=(
  repo-review-code
  repo-review-security
  repo-review-tests
  repo-review-docs
  repo-review-synthesis
)

for skill in "${specialists[@]}"; do
  [[ -f "${skills_root}/${skill}/SKILL.md" ]]
  [[ -f "${skills_root}/${skill}/adl-skill.yaml" ]]
  [[ -f "${skills_root}/${skill}/agents/openai.yaml" ]]
  grep -Fq 'reference_doc: "../docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"' "${skills_root}/${skill}/adl-skill.yaml"
  grep -Fq "allow_code_edits: false" "${skills_root}/${skill}/adl-skill.yaml"
  grep -Fq "allow_network: false" "${skills_root}/${skill}/adl-skill.yaml"
done

grep -Fq 'id: "repo-review-code"' "${skills_root}/repo-review-code/adl-skill.yaml"
grep -Fq 'id: "repo-review-security"' "${skills_root}/repo-review-security/adl-skill.yaml"
grep -Fq 'id: "repo-review-tests"' "${skills_root}/repo-review-tests/adl-skill.yaml"
grep -Fq 'id: "repo-review-docs"' "${skills_root}/repo-review-docs/adl-skill.yaml"
grep -Fq 'id: "repo-review-synthesis"' "${skills_root}/repo-review-synthesis/adl-skill.yaml"

grep -Fq "behavioral bugs, regressions" "${skills_root}/repo-review-code/SKILL.md"
grep -Fq "trust boundaries, secret handling" "${skills_root}/repo-review-security/SKILL.md"
grep -Fq "missing coverage, weak assertions" "${skills_root}/repo-review-tests/SKILL.md"
grep -Fq "misleading docs, stale commands" "${skills_root}/repo-review-docs/SKILL.md"
grep -Fq "without hiding severity, disagreement" "${skills_root}/repo-review-synthesis/SKILL.md"

grep -Fq "repo_review_code.v1" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"
grep -Fq "repo_review_synthesis.v1" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"
grep -Fq "Preserve the highest severity" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"
grep -Fq "Use the existing \`repo-code-review\` skill" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/repo-review-code/SKILL.md" \
  "${skills_root}/repo-review-security/SKILL.md" \
  "${skills_root}/repo-review-tests/SKILL.md" \
  "${skills_root}/repo-review-docs/SKILL.md" \
  "${skills_root}/repo-review-synthesis/SKILL.md"

export CODEX_HOME="${tmpdir}/codex-home"
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null
for skill in "${specialists[@]}"; do
  [[ -f "${CODEX_HOME}/skills/${skill}/SKILL.md" ]]
done

echo "PASS test_multi_agent_repo_review_skill_suite_contracts"
