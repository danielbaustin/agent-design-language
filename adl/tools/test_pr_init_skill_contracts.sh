#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/pr-init/SKILL.md" ]]
[[ -f "${skills_root}/pr-init/adl-skill.yaml" ]]
[[ -f "${skills_root}/pr-init/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/PR_INIT_SKILL_INPUT_SCHEMA.md" ]]
[[ -f "${repo_root}/docs/templates/PR_INIT_INVOCATION_TEMPLATE.md" ]]

grep -Fq "handle exactly one issue target per invocation" "${skills_root}/pr-init/SKILL.md"
grep -Fq "one subagent per issue" "${skills_root}/pr-init/SKILL.md"
grep -Fq 'allow_subagents: true' "${skills_root}/pr-init/adl-skill.yaml"
grep -Fq 'exactly_one_issue_target_per_invocation' "${skills_root}/pr-init/adl-skill.yaml"
grep -Fq 'one_issue_per_invocation_parent_aggregates_multi_issue_results' "${skills_root}/pr-init/adl-skill.yaml"
grep -Fq "Do not report multiple issues in one \`pr-init\` result." "${skills_root}/pr-init/references/output-contract.md"
grep -Fq "one issue target per invocation" "${skills_root}/docs/PR_INIT_SKILL_INPUT_SCHEMA.md"
grep -Fq "one validated payload per issue" "${repo_root}/docs/templates/PR_INIT_INVOCATION_TEMPLATE.md"
grep -Fq "one \`pr-init\` invocation per issue" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

echo "PASS test_pr_init_skill_contracts"
