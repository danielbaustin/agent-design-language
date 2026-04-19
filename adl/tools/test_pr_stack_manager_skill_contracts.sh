#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
docs_root="${skills_root}/docs"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/pr-stack-manager/SKILL.md" ]]
[[ -f "${skills_root}/pr-stack-manager/adl-skill.yaml" ]]
[[ -f "${skills_root}/pr-stack-manager/agents/openai.yaml" ]]
[[ -f "${skills_root}/pr-stack-manager/references/output-contract.md" ]]
[[ -f "${skills_root}/pr-stack-manager/references/pr-stack-manager-playbook.md" ]]
[[ -f "${docs_root}/PR_STACK_MANAGER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'name: pr-stack-manager' "${skills_root}/pr-stack-manager/SKILL.md"
grep -Fq 'id: "pr-stack-manager"' "${skills_root}/pr-stack-manager/adl-skill.yaml"
grep -Fq 'id: "pr_stack_manager.v1"' "${skills_root}/pr-stack-manager/adl-skill.yaml"
grep -Fq 'pr_stack_manager.v1' "${docs_root}/PR_STACK_MANAGER_SKILL_INPUT_SCHEMA.md"
grep -Fq "dependency_graph" "${skills_root}/pr-stack-manager/references/output-contract.md"
grep -Fq "Repair Policy" "${skills_root}/pr-stack-manager/references/pr-stack-manager-playbook.md"
grep -Fq "pr-stack-manager" "${docs_root}/OPERATIONAL_SKILLS_GUIDE.md"

CODEX_HOME="${tmpdir}/codex-home" \
ADL_OPERATIONAL_SKILLS_SOURCE_ROOT="${skills_root}" \
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/tmp/pr-stack-manager-install.out
[[ -f "${tmpdir}/codex-home/skills/pr-stack-manager/SKILL.md" ]]
diff -q \
  "${skills_root}/pr-stack-manager/SKILL.md" \
  "${tmpdir}/codex-home/skills/pr-stack-manager/SKILL.md" >/dev/null

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/pr-stack-manager/SKILL.md"

echo "PASS test_pr_stack_manager_skill_contracts"
