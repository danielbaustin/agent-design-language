#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/pr-closeout/SKILL.md" ]]
[[ -f "${skills_root}/pr-closeout/adl-skill.yaml" ]]
[[ -f "${skills_root}/pr-closeout/agents/openai.yaml" ]]
[[ -f "${skills_root}/pr-closeout/references/closeout-playbook.md" ]]
[[ -f "${skills_root}/pr-closeout/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/PR_CLOSEOUT_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq "post-merge and post-closure cleanup phase" "${skills_root}/pr-closeout/SKILL.md"
grep -Fq "prune the local worktree safely" "${skills_root}/pr-closeout/SKILL.md"
grep -Fq "pr-closeout" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "pr-closeout" "${skills_root}/pr-finish/SKILL.md"

echo "PASS test_pr_closeout_skill_contracts"
