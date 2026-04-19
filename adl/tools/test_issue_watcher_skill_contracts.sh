#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/issue-watcher/SKILL.md" ]]
[[ -f "${skills_root}/issue-watcher/adl-skill.yaml" ]]
[[ -f "${skills_root}/issue-watcher/agents/openai.yaml" ]]
[[ -f "${skills_root}/issue-watcher/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/ISSUE_WATCHER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "issue-watcher"' "${skills_root}/issue-watcher/adl-skill.yaml"
grep -Fq 'id: "issue_watcher.v1"' "${skills_root}/issue-watcher/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/ISSUE_WATCHER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/issue-watcher/adl-skill.yaml"
grep -Fq "policy.stop_after_watch_must_be_true" "${skills_root}/issue-watcher/adl-skill.yaml"
grep -Fq "pr_repair_must_route_to_pr_janitor" "${skills_root}/issue-watcher/adl-skill.yaml"
grep -Fq "Watch one issue, PR, branch, or dependency gate" "${skills_root}/issue-watcher/SKILL.md"
grep -Fq "route to \`pr-janitor\`" "${skills_root}/issue-watcher/SKILL.md"
grep -Fq 'Schema id: `issue_watcher.v1`' "${skills_root}/docs/ISSUE_WATCHER_SKILL_INPUT_SCHEMA.md"
grep -Fq "watch_issue | watch_pr | watch_pr_url | watch_branch | watch_dependency_gate" "${skills_root}/docs/ISSUE_WATCHER_SKILL_INPUT_SCHEMA.md"
grep -Fq "route_blockers: true" "${skills_root}/docs/ISSUE_WATCHER_SKILL_INPUT_SCHEMA.md"
grep -Fq "handoff.next_skill" "${skills_root}/issue-watcher/references/output-contract.md"
grep -Fq "If PR checks, conflicts, or requested changes are the blocker" "${skills_root}/issue-watcher/references/output-contract.md"
grep -Fq "issue_watcher.v1" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/issue-watcher/SKILL.md"

echo "PASS test_issue_watcher_skill_contracts"
