#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
contract="${repo_root}/docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "$contract" ]]

grep -Fq "# Issue Lifecycle Shepherd Contract" "$contract"
grep -Fq "The canonical issue-lifecycle shepherd states are:" "$contract"
grep -Fq "9. \`blocked\`" "$contract"
grep -Fq "any_state -> blocked" "$contract"
grep -Fq "merge_authority_human_only" "$contract"
grep -Fq "workflow-conductor" "$contract"
grep -Fq "issue-watcher" "$contract"
grep -Fq "pr-janitor" "$contract"
grep -Fq "pr-closeout" "$contract"
grep -Fq "next_skill: pr-init | pr-ready | pr-run | pr-finish | issue-watcher | pr-janitor | pr-closeout | stp-editor | sip-editor | spp-editor | srp-editor | sor-editor | human_review | none" "$contract"

grep -Fq "ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md" \
  "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md" \
  "${skills_root}/workflow-conductor/references/output-contract.md"
grep -Fq "ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md" \
  "${skills_root}/issue-watcher/references/output-contract.md"
grep -Fq "ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md" \
  "${skills_root}/pr-janitor/references/output-contract.md"
grep -Fq "ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md" \
  "${skills_root}/pr-closeout/references/output-contract.md"
grep -Fq "ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md" \
  "${skills_root}/pr-finish/references/output-contract.md"
grep -Fq "ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md" \
  "${repo_root}/docs/default_workflow.md"
grep -Fq "issue-watcher\` for healthy waiting states and through \`pr-janitor\` only when" \
  "${repo_root}/docs/default_workflow.md"

echo "PASS test_issue_lifecycle_shepherd_contract"
