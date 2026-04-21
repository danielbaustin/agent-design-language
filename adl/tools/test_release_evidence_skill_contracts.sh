#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/release-evidence/SKILL.md" ]]
[[ -f "${skills_root}/release-evidence/adl-skill.yaml" ]]
[[ -f "${skills_root}/release-evidence/agents/openai.yaml" ]]
[[ -f "${skills_root}/release-evidence/references/release-evidence-playbook.md" ]]
[[ -f "${skills_root}/release-evidence/references/output-contract.md" ]]
[[ -x "${skills_root}/release-evidence/scripts/assemble_release_evidence.py" ]]
[[ -f "${skills_root}/docs/RELEASE_EVIDENCE_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "release-evidence"' "${skills_root}/release-evidence/adl-skill.yaml"
grep -Fq 'id: "release_evidence.v1"' "${skills_root}/release-evidence/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/RELEASE_EVIDENCE_SKILL_INPUT_SCHEMA.md"' "${skills_root}/release-evidence/adl-skill.yaml"
grep -Fq "stop_before_release_approval_must_be_true" "${skills_root}/release-evidence/adl-skill.yaml"
grep -Fq "Assemble milestone release proof surfaces" "${skills_root}/release-evidence/SKILL.md"
grep -Fq "This report does not approve the release" "${skills_root}/release-evidence/references/output-contract.md"
grep -Fq "Schema id: \`release_evidence.v1\`" "${skills_root}/docs/RELEASE_EVIDENCE_SKILL_INPUT_SCHEMA.md"

milestone_root="${tmpdir}/milestone"
report_root="${tmpdir}/release-evidence-report"
mkdir -p "${milestone_root}"
cat >"${milestone_root}/README.md" <<'MD'
# vTEST

This milestone has an issue wave and PR review surface.
MD
cat >"${milestone_root}/WBS_vTEST.md" <<'MD'
# WBS

WP issue and PR mapping is recorded here.
MD
cat >"${milestone_root}/DEMO_MATRIX_vTEST.md" <<'MD'
# Demo Matrix

Demo proof coverage exists for the milestone.
MD
cat >"${milestone_root}/INTERNAL_REVIEW_vTEST.md" <<'MD'
# Internal Review

Review findings were triaged.
MD
cat >"${milestone_root}/REMEDIATION_vTEST.md" <<'MD'
# Remediation

Finding follow-up status is recorded.
MD
cat >"${milestone_root}/MILESTONE_CHECKLIST_vTEST.md" <<'MD'
# Checklist

- [x] Validation command log recorded.
- [ ] External ceremony not run yet.
MD

python3 "${skills_root}/release-evidence/scripts/assemble_release_evidence.py" \
  --milestone vTEST \
  --milestone-root "${milestone_root}" \
  --out "${report_root}" \
  --run-id release-evidence-contract-test >/tmp/release-evidence.out

[[ -f "${report_root}/release_evidence_report.json" ]]
[[ -f "${report_root}/release_evidence_report.md" ]]
grep -Fq '"schema": "adl.release_evidence_report.v1"' "${report_root}/release_evidence_report.json"
grep -Fq '"run_id": "release-evidence-contract-test"' "${report_root}/release_evidence_report.json"
grep -Fq '"status": "partial"' "${report_root}/release_evidence_report.json"
grep -Fq '"release_approved": false' "${report_root}/release_evidence_report.json"
grep -Fq '"mutated_repository": false' "${report_root}/release_evidence_report.json"
grep -Fq "## Evidence Families" "${report_root}/release_evidence_report.md"
grep -Fq "## Non-Claims" "${report_root}/release_evidence_report.md"
grep -Fq "## Safety Flags" "${report_root}/release_evidence_report.md"
grep -Fq "release_approved: false" "${report_root}/release_evidence_report.md"

missing_report="${tmpdir}/missing-report"
python3 "${skills_root}/release-evidence/scripts/assemble_release_evidence.py" \
  --milestone vMISSING \
  --milestone-root "${tmpdir}/does-not-exist" \
  --out "${missing_report}" \
  --run-id release-evidence-missing-test >/tmp/release-evidence-missing.out
grep -Fq '"status": "not_run"' "${missing_report}/release_evidence_report.json"

if grep -R "${tmpdir}" "${report_root}" "${missing_report}" >/dev/null; then
  echo "release evidence artifacts should not leak absolute temp paths" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/release-evidence/SKILL.md"

echo "PASS test_release_evidence_skill_contracts"

