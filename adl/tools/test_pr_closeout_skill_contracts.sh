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
grep -Fq "record deferral, supersession, or duplicate links" "${skills_root}/pr-closeout/SKILL.md"
grep -Fq 'id: "pr_closeout.v1"' "${skills_root}/pr-closeout/adl-skill.yaml"
grep -Fq 'reference_doc: "/Users/daniel/git/agent-design-language/adl/tools/skills/docs/PR_CLOSEOUT_SKILL_INPUT_SCHEMA.md"' "${skills_root}/pr-closeout/adl-skill.yaml"
grep -Fq "policy.closure_outcome_must_be_explicit" "${skills_root}/pr-closeout/adl-skill.yaml"
grep -Fq "policy.closure_references_must_be_present_when_closure_outcome_is_superseded_or_duplicate" "${skills_root}/pr-closeout/adl-skill.yaml"
grep -Fq "closeout_pr" "${skills_root}/docs/PR_CLOSEOUT_SKILL_INPUT_SCHEMA.md"
grep -Fq "requires \`target.issue_number\`" "${skills_root}/docs/PR_CLOSEOUT_SKILL_INPUT_SCHEMA.md"
grep -Fq "pr-closeout" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "pr-closeout" "${skills_root}/pr-finish/SKILL.md"
grep -Fq "after the PR outcome or explicit non-PR closure disposition is settled" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

echo "PASS test_pr_closeout_skill_contracts"
