#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/workflow-conductor/SKILL.md" ]]
[[ -f "${skills_root}/workflow-conductor/adl-skill.yaml" ]]
[[ -f "${skills_root}/workflow-conductor/agents/openai.yaml" ]]
[[ -f "${skills_root}/workflow-conductor/references/conductor-playbook.md" ]]
[[ -f "${skills_root}/workflow-conductor/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq "thin orchestrator" "${skills_root}/workflow-conductor/SKILL.md"
grep -Fq "stop after routing and compliance recording" "${skills_root}/workflow-conductor/SKILL.md"
grep -Fq 'id: "workflow_conductor.v1"' "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq 'reference_doc: "/Users/daniel/git/agent-design-language/adl/tools/skills/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "policy.stop_after_routing_must_be_true" "${skills_root}/workflow-conductor/adl-skill.yaml"
grep -Fq "route_issue" "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "requires \`target.issue_number\`" "${skills_root}/docs/WORKFLOW_CONDUCTOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "workflow-conductor" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "resume from partially completed early steps" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"

echo "PASS test_workflow_conductor_skill_contracts"
