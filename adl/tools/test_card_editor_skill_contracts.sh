#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"

for skill in stp-editor sip-editor sor-editor; do
  [[ -f "${skills_root}/${skill}/SKILL.md" ]]
  [[ -f "${skills_root}/${skill}/adl-skill.yaml" ]]
  [[ -f "${skills_root}/${skill}/agents/openai.yaml" ]]
  [[ -f "${skills_root}/${skill}/references/edit-playbook.md" ]]
  [[ -f "${skills_root}/${skill}/references/output-contract.md" ]]
done

[[ -f "${skills_root}/docs/STP_EDITOR_SKILL_INPUT_SCHEMA.md" ]]
[[ -f "${skills_root}/docs/SIP_EDITOR_SKILL_INPUT_SCHEMA.md" ]]
[[ -f "${skills_root}/docs/SOR_EDITOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq "must not:" "${skills_root}/stp-editor/SKILL.md"
grep -Fq "must not:" "${skills_root}/sip-editor/SKILL.md"
grep -Fq "must not:" "${skills_root}/sor-editor/SKILL.md"
grep -Fq "invent validation that did not happen" "${skills_root}/sor-editor/SKILL.md"
grep -Fq 'fix truthful `Branch` state such as `not bound yet` vs bound execution branch' "${skills_root}/sip-editor/SKILL.md"
grep -Fq "tighten clarity around goal, required outcome, acceptance criteria, and scope" "${skills_root}/stp-editor/SKILL.md"

grep -Fq "stp-editor" "${skills_root}/pr-init/SKILL.md"
grep -Fq "sip-editor" "${skills_root}/pr-ready/SKILL.md"
grep -Fq "sor-editor" "${skills_root}/pr-run/SKILL.md"
grep -Fq "sor-editor" "${skills_root}/pr-finish/SKILL.md"

echo "PASS test_card_editor_skill_contracts"
