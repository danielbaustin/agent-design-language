#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/demo-operator/SKILL.md" ]]
[[ -f "${skills_root}/demo-operator/adl-skill.yaml" ]]
[[ -f "${skills_root}/demo-operator/agents/openai.yaml" ]]
[[ -f "${skills_root}/demo-operator/references/demo-playbook.md" ]]
[[ -f "${skills_root}/demo-operator/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/DEMO_OPERATOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "demo_operator.v1"' "${skills_root}/demo-operator/adl-skill.yaml"
grep -Fq 'reference_doc: "/Users/daniel/git/agent-design-language/adl/tools/skills/docs/DEMO_OPERATOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/demo-operator/adl-skill.yaml"
grep -Fq "policy.classification_mode_must_be_explicit" "${skills_root}/demo-operator/adl-skill.yaml"
grep -Fq "policy.validation_mode_must_be_explicit" "${skills_root}/demo-operator/adl-skill.yaml"
grep -Fq "run one named demo" "${skills_root}/demo-operator/SKILL.md"
grep -Fq "proving, non-proving, skipped, or failed" "${skills_root}/demo-operator/SKILL.md"
grep -Fq "demo_operator.v1" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "operate_named_demo | operate_demo_command | operate_demo_doc" "${skills_root}/docs/DEMO_OPERATOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "stop_after_operation: true" "${skills_root}/docs/DEMO_OPERATOR_SKILL_INPUT_SCHEMA.md"

echo "PASS test_demo_operator_skill_contracts"
