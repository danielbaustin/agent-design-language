#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/documentation-specialist/SKILL.md" ]]
[[ -f "${skills_root}/documentation-specialist/adl-skill.yaml" ]]
[[ -f "${skills_root}/documentation-specialist/agents/openai.yaml" ]]
[[ -f "${skills_root}/documentation-specialist/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/DOCUMENTATION_SPECIALIST_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'name: documentation-specialist' "${skills_root}/documentation-specialist/SKILL.md"
grep -Fq 'id: "documentation-specialist"' "${skills_root}/documentation-specialist/adl-skill.yaml"
grep -Fq 'id: "documentation_specialist.v1"' "${skills_root}/documentation-specialist/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/DOCUMENTATION_SPECIALIST_SKILL_INPUT_SCHEMA.md"' "${skills_root}/documentation-specialist/adl-skill.yaml"
grep -Fq "policy.bounded_target_required_must_be_true" "${skills_root}/documentation-specialist/adl-skill.yaml"
grep -Fq "policy.source_evidence_required_must_be_true" "${skills_root}/documentation-specialist/adl-skill.yaml"
grep -Fq "policy.stop_before_publication_must_be_true" "${skills_root}/documentation-specialist/adl-skill.yaml"
grep -Fq "policy.stop_before_broad_rewrite_must_be_true" "${skills_root}/documentation-specialist/adl-skill.yaml"
grep -Fq "Plan, write, audit, repair, or polish bounded repository documentation" "${skills_root}/documentation-specialist/SKILL.md"
grep -Fq "Classify each claim" "${skills_root}/documentation-specialist/SKILL.md"
grep -Fq "Do not infer a repo-wide rewrite" "${skills_root}/documentation-specialist/SKILL.md"
grep -Fq "Publication attempted: true | false" "${skills_root}/documentation-specialist/references/output-contract.md"
grep -Fq "Release approval claimed: false." "${skills_root}/documentation-specialist/references/output-contract.md"
grep -Fq "Schema id: \`documentation_specialist.v1\`" "${skills_root}/docs/DOCUMENTATION_SPECIALIST_SKILL_INPUT_SCHEMA.md"
grep -Fq "README, milestone, feature, ADR, demo, review, architecture" "${skills_root}/docs/DOCUMENTATION_SPECIALIST_SKILL_INPUT_SCHEMA.md"
grep -Fq "Documentation Specialist" "${skills_root}/documentation-specialist/agents/openai.yaml"

if grep -R "${tmpdir}" \
  "${skills_root}/documentation-specialist" \
  "${skills_root}/docs/DOCUMENTATION_SPECIALIST_SKILL_INPUT_SCHEMA.md" >/dev/null; then
  echo "documentation specialist skill should not leak absolute temp paths" >&2
  exit 1
fi

CODEX_HOME="${tmpdir}/codex-home" \
ADL_OPERATIONAL_SKILLS_SOURCE_ROOT="${skills_root}" \
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/tmp/documentation-specialist-install.out
[[ -f "${tmpdir}/codex-home/skills/documentation-specialist/SKILL.md" ]]
[[ -f "${tmpdir}/codex-home/skills/documentation-specialist/adl-skill.yaml" ]]
diff -q \
  "${skills_root}/documentation-specialist/SKILL.md" \
  "${tmpdir}/codex-home/skills/documentation-specialist/SKILL.md" >/dev/null

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/documentation-specialist/SKILL.md"

echo "PASS test_documentation_specialist_skill_contracts"
