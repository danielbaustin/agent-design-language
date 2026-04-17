#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/repo-packet-builder/SKILL.md" ]]
[[ -f "${skills_root}/repo-packet-builder/adl-skill.yaml" ]]
[[ -f "${skills_root}/repo-packet-builder/agents/openai.yaml" ]]
[[ -f "${skills_root}/repo-packet-builder/references/packet-playbook.md" ]]
[[ -f "${skills_root}/repo-packet-builder/references/output-contract.md" ]]
[[ -x "${skills_root}/repo-packet-builder/scripts/build_repo_packet.py" ]]
[[ -f "${skills_root}/docs/REPO_PACKET_BUILDER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "repo-packet-builder"' "${skills_root}/repo-packet-builder/adl-skill.yaml"
grep -Fq 'id: "repo_packet_builder.v1"' "${skills_root}/repo-packet-builder/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REPO_PACKET_BUILDER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/repo-packet-builder/adl-skill.yaml"
grep -Fq "policy.stop_before_review_must_be_true" "${skills_root}/repo-packet-builder/adl-skill.yaml"
grep -Fq "packet-construction skill, not a reviewer" "${skills_root}/repo-packet-builder/SKILL.md"
grep -Fq "scripts/build_repo_packet.py" "${skills_root}/repo-packet-builder/SKILL.md"
grep -Fq "Do not write absolute host paths" "${skills_root}/repo-packet-builder/SKILL.md"
grep -Fq "Specialist Lane Hints" "${skills_root}/repo-packet-builder/references/packet-playbook.md"
grep -Fq "run_manifest.json" "${skills_root}/repo-packet-builder/references/output-contract.md"
grep -Fq "Schema id: \`repo_packet_builder.v1\`" "${skills_root}/docs/REPO_PACKET_BUILDER_SKILL_INPUT_SCHEMA.md"

packet_root="${tmpdir}/packet"
python3 "${skills_root}/repo-packet-builder/scripts/build_repo_packet.py" "${repo_root}" --out "${packet_root}" >/tmp/repo-packet-builder.out
[[ -f "${packet_root}/run_manifest.json" ]]
[[ -f "${packet_root}/repo_scope.md" ]]
[[ -f "${packet_root}/repo_inventory.json" ]]
[[ -f "${packet_root}/evidence_index.json" ]]
[[ -f "${packet_root}/specialist_assignments.json" ]]
grep -Fq '"schema": "codebuddy.repo_packet.run_manifest.v1"' "${packet_root}/run_manifest.json"
grep -Fq '# Repo Scope' "${packet_root}/repo_scope.md"
grep -Fq '"manifests"' "${packet_root}/repo_inventory.json"
grep -Fq '"evidence"' "${packet_root}/evidence_index.json"
grep -Fq '"assignments"' "${packet_root}/specialist_assignments.json"
if grep -R "${repo_root}" "${packet_root}" >/dev/null; then
  echo "repo packet should not leak absolute repo root" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/repo-packet-builder/SKILL.md"

echo "PASS test_repo_packet_builder_skill_contracts"
