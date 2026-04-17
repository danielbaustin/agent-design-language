#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/redaction-and-evidence-auditor/SKILL.md" ]]
[[ -f "${skills_root}/redaction-and-evidence-auditor/adl-skill.yaml" ]]
[[ -f "${skills_root}/redaction-and-evidence-auditor/agents/openai.yaml" ]]
[[ -f "${skills_root}/redaction-and-evidence-auditor/references/redaction-playbook.md" ]]
[[ -f "${skills_root}/redaction-and-evidence-auditor/references/output-contract.md" ]]
[[ -x "${skills_root}/redaction-and-evidence-auditor/scripts/audit_review_packet.py" ]]
[[ -f "${skills_root}/docs/REDACTION_AND_EVIDENCE_AUDITOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "redaction-and-evidence-auditor"' "${skills_root}/redaction-and-evidence-auditor/adl-skill.yaml"
grep -Fq 'id: "redaction_and_evidence_auditor.v1"' "${skills_root}/redaction-and-evidence-auditor/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REDACTION_AND_EVIDENCE_AUDITOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/redaction-and-evidence-auditor/adl-skill.yaml"
grep -Fq "policy.stop_before_mutation_must_be_true" "${skills_root}/redaction-and-evidence-auditor/adl-skill.yaml"
grep -Fq "safety gate, not a reviewer" "${skills_root}/redaction-and-evidence-auditor/SKILL.md"
grep -Fq "scripts/audit_review_packet.py" "${skills_root}/redaction-and-evidence-auditor/SKILL.md"
grep -Fq "Do not include full secret values" "${skills_root}/redaction-and-evidence-auditor/references/output-contract.md"
grep -Fq "Schema id: \`redaction_and_evidence_auditor.v1\`" "${skills_root}/docs/REDACTION_AND_EVIDENCE_AUDITOR_SKILL_INPUT_SCHEMA.md"

packet_root="${tmpdir}/packet"
audit_root="${tmpdir}/audit"
mkdir -p "${packet_root}"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "contract-test",
  "publication_allowed": true,
  "privacy_mode": "local_only"
}
JSON
cat >"${packet_root}/repo_scope.md" <<'EOF'
# Repo Scope

This packet uses repo-relative paths only.
EOF

python3 "${skills_root}/redaction-and-evidence-auditor/scripts/audit_review_packet.py" \
  "${packet_root}" --out "${audit_root}" >/tmp/redaction-auditor-pass.out
[[ -f "${audit_root}/redaction_report.json" ]]
[[ -f "${audit_root}/redaction_report.md" ]]
grep -Fq '"status": "pass"' "${audit_root}/redaction_report.json"
grep -Fq '"publication_recommendation": "allow_internal"' "${audit_root}/redaction_report.json"
if grep -R "${tmpdir}" "${audit_root}" >/dev/null; then
  echo "audit output should not leak absolute temp paths" >&2
  exit 1
fi

unsafe_packet="${tmpdir}/unsafe-packet"
unsafe_audit="${tmpdir}/unsafe-audit"
mkdir -p "${unsafe_packet}"
cat >"${unsafe_packet}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "unsafe-contract-test",
  "publication_allowed": false,
  "privacy_mode": "local_only"
}
JSON
cat >"${unsafe_packet}/review.md" <<'EOF'
The tool emitted OPENAI_API_KEY=sk-test-redaction-example-000000000000
The local file path was /Users/example/project/private.txt
See http://127.0.0.1:8080/debug for details.
EOF

python3 "${skills_root}/redaction-and-evidence-auditor/scripts/audit_review_packet.py" \
  "${unsafe_packet}" --out "${unsafe_audit}" --audience customer_private >/tmp/redaction-auditor-fail.out
grep -Fq '"status": "fail"' "${unsafe_audit}/redaction_report.json"
grep -Fq '"publication_recommendation": "block_publication"' "${unsafe_audit}/redaction_report.json"
grep -Fq '"category": "credential_assignment"' "${unsafe_audit}/redaction_report.json"
grep -Fq '"category": "private_host_path"' "${unsafe_audit}/redaction_report.json"
grep -Fq '"category": "internal_url"' "${unsafe_audit}/redaction_report.json"
if grep -R "sk-test-redaction-example-000000000000" "${unsafe_audit}" >/dev/null; then
  echo "audit output should mask secret-like values" >&2
  exit 1
fi
[[ -f "${unsafe_audit}/blocked_publication_note.md" ]]

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/redaction-and-evidence-auditor/SKILL.md"

echo "PASS test_redaction_and_evidence_auditor_skill_contracts"
