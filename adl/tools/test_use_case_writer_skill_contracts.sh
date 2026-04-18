#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/use-case-writer/SKILL.md" ]]
[[ -f "${skills_root}/use-case-writer/adl-skill.yaml" ]]
[[ -f "${skills_root}/use-case-writer/agents/openai.yaml" ]]
[[ -f "${skills_root}/use-case-writer/references/use-case-writing-playbook.md" ]]
[[ -f "${skills_root}/use-case-writer/references/output-contract.md" ]]
[[ -x "${skills_root}/use-case-writer/scripts/write_use_cases.py" ]]
[[ -f "${skills_root}/docs/USE_CASE_WRITER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "use-case-writer"' "${skills_root}/use-case-writer/adl-skill.yaml"
grep -Fq 'id: "use_case_writer.v1"' "${skills_root}/use-case-writer/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/USE_CASE_WRITER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/use-case-writer/adl-skill.yaml"
grep -Fq "declared_source_must_be_explicit" "${skills_root}/use-case-writer/adl-skill.yaml"
grep -Fq "without inventing product commitments" "${skills_root}/use-case-writer/SKILL.md"
grep -Fq "Flag unsupported assumptions" "${skills_root}/use-case-writer/references/output-contract.md"
grep -Fq "Schema id: \`use_case_writer.v1\`" "${skills_root}/docs/USE_CASE_WRITER_SKILL_INPUT_SCHEMA.md"

use_case_root="${tmpdir}/use-case"
packet_root="${tmpdir}/use-case-packet"
mkdir -p "${use_case_root}"
cat >"${use_case_root}/use_case_manifest.json" <<'JSON'
{
  "run_id": "use-case-writer-contract-test",
  "mode": "write_demo_use_cases",
  "source_ref": "issue-2040",
  "audience": "milestone reviewer"
}
JSON
cat >"${use_case_root}/source_brief.md" <<'MD'
# Source Brief

- Source: issue-2040
- Audience: milestone reviewer

Write grounded use cases for a demo planning packet without creating issues or claiming implementation completion.
MD
cat >"${use_case_root}/actors.md" <<'MD'
# Actors

## Actors

- Operator
- Reviewer
MD
cat >"${use_case_root}/goals.md" <<'MD'
# Goals

## User Goals

- Operator turns a demo idea into source-grounded scenarios.
- Reviewer checks acceptance hooks without chasing implicit assumptions.
MD
cat >"${use_case_root}/system_behavior.md" <<'MD'
# System Behavior

## System Behavior

- The skill emits actor flows, acceptance hooks, non-goals, and unsupported assumptions.
- The skill keeps unsupported product claims out of the use-case packet.
MD
cat >"${use_case_root}/acceptance_hooks.md" <<'MD'
# Acceptance Hooks

## Acceptance Hooks

- Packet names actors and user goals separately from system behavior.
- Packet records unsupported assumptions instead of promoting them to requirements.
MD
cat >"${use_case_root}/non_goals.md" <<'MD'
# Non-goals

## Non-goals

- Creating GitHub issues directly.
- Replacing implementation planning.
MD
cat >"${use_case_root}/unsupported_assumptions.md" <<'MD'
# Unsupported Assumptions

## Unsupported Assumptions

- Publication scope is not defined by the source brief.
MD

python3 "${skills_root}/use-case-writer/scripts/write_use_cases.py" \
  "${use_case_root}" --out "${packet_root}" --max-use-cases 2 >/tmp/use-case-writer.out
[[ -f "${packet_root}/use_case_packet.json" ]]
[[ -f "${packet_root}/use_case_packet.md" ]]
grep -Fq '"schema": "adl.use_case_packet.v1"' "${packet_root}/use_case_packet.json"
grep -Fq '"run_id": "use-case-writer-contract-test"' "${packet_root}/use_case_packet.json"
grep -Fq '"status": "ready"' "${packet_root}/use_case_packet.json"
grep -Fq '"created_issues": false' "${packet_root}/use_case_packet.json"
grep -Fq '"mutated_repository": false' "${packet_root}/use_case_packet.json"
grep -Fq "## Use Case Packet Summary" "${packet_root}/use_case_packet.md"
grep -Fq "## Unsupported Assumptions" "${packet_root}/use_case_packet.md"
grep -Fq "Created issues: false." "${packet_root}/use_case_packet.md"
grep -Fq "Mutated repository: false." "${packet_root}/use_case_packet.md"

missing_root="${tmpdir}/missing-source"
mkdir -p "${missing_root}"
python3 "${skills_root}/use-case-writer/scripts/write_use_cases.py" \
  "${missing_root}" --out "${tmpdir}/missing-packet" >/tmp/use-case-writer-missing.out
grep -Fq '"status": "not_run"' "${tmpdir}/missing-packet/use_case_packet.json"

if grep -R "${tmpdir}" "${packet_root}" "${tmpdir}/missing-packet" >/dev/null; then
  echo "use-case writer artifacts should not leak absolute temp paths" >&2
  exit 1
fi
if find "${packet_root}" -name '*issue*.md' -o -name '*.rs' -o -name '*.py' | grep -q .; then
  echo "use-case writer should not create issues or implementation files" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/use-case-writer/SKILL.md"

export CODEX_HOME="${tmpdir}/codex-home"
bash "${repo_root}/adl/tools/install_adl_operational_skills.sh" >/dev/null
[[ -f "${CODEX_HOME}/skills/use-case-writer/SKILL.md" ]]
[[ -f "${CODEX_HOME}/skills/use-case-writer/adl-skill.yaml" ]]
[[ -f "${CODEX_HOME}/skills/use-case-writer/agents/openai.yaml" ]]

echo "PASS test_use_case_writer_skill_contracts"
