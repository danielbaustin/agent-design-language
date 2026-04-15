#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

[[ -f "${skills_root}/test-generator/SKILL.md" ]]
[[ -f "${skills_root}/test-generator/adl-skill.yaml" ]]
[[ -f "${skills_root}/test-generator/agents/openai.yaml" ]]
[[ -f "${skills_root}/test-generator/references/test-playbook.md" ]]
[[ -f "${skills_root}/test-generator/references/output-contract.md" ]]
[[ -f "${skills_root}/docs/TEST_GENERATOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "test_generator.v1"' "${skills_root}/test-generator/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/TEST_GENERATOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/test-generator/adl-skill.yaml"
grep -Fq "policy.test_depth_must_be_explicit" "${skills_root}/test-generator/adl-skill.yaml"
grep -Fq "policy.validation_mode_must_be_explicit" "${skills_root}/test-generator/adl-skill.yaml"
grep -Fq "bounded regression-test authoring" "${skills_root}/test-generator/SKILL.md"
grep -Fq "smallest truthful test surface" "${skills_root}/test-generator/SKILL.md"
grep -Fq "focused tests for a concrete issue, diff, file, or worktree" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "test_generator.v1" "${skills_root}/docs/OPERATIONAL_SKILLS_GUIDE.md"
grep -Fq "generate_for_issue | generate_for_diff | generate_for_path | generate_for_worktree" "${skills_root}/docs/TEST_GENERATOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "stop_after_generation: true" "${skills_root}/docs/TEST_GENERATOR_SKILL_INPUT_SCHEMA.md"

echo "PASS test_test_generator_skill_contracts"
