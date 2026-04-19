#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
docs_root="${skills_root}/docs"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/records-hygiene/SKILL.md" ]]
[[ -f "${skills_root}/records-hygiene/adl-skill.yaml" ]]
[[ -f "${skills_root}/records-hygiene/agents/openai.yaml" ]]
[[ -f "${skills_root}/records-hygiene/references/output-contract.md" ]]
[[ -f "${skills_root}/records-hygiene/references/records-hygiene-playbook.md" ]]
[[ -f "${skills_root}/docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'name: records-hygiene' "${skills_root}/records-hygiene/SKILL.md"
grep -Fq 'id: "records-hygiene"' "${skills_root}/records-hygiene/adl-skill.yaml"
grep -Fq 'id: "records_hygiene.v1"' "${skills_root}/records-hygiene/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md"' "${skills_root}/records-hygiene/adl-skill.yaml"
grep -Fq 'records_hygiene.v1' "${skills_root}/docs/RECORDS_HYGIENE_SKILL_INPUT_SCHEMA.md"
grep -Fq "safe_repairs_applied" "${skills_root}/records-hygiene/references/output-contract.md"
grep -Fq "records-hygiene" "${docs_root}/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "placeholder" "${skills_root}/records-hygiene/references/records-hygiene-playbook.md"

CODEX_HOME="${tmpdir}/codex-home" \
ADL_OPERATIONAL_SKILLS_SOURCE_ROOT="${skills_root}" \
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/tmp/records-hygiene-install.out
[[ -f "${tmpdir}/codex-home/skills/records-hygiene/SKILL.md" ]]
diff -q \
  "${skills_root}/records-hygiene/SKILL.md" \
  "${tmpdir}/codex-home/skills/records-hygiene/SKILL.md" >/dev/null

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/records-hygiene/SKILL.md"

echo "PASS test_records_hygiene_skill_contracts"
