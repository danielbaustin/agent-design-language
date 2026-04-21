#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

skill_root="${skills_root}/review-readiness-cleanup"
python_script="${skill_root}/scripts/inspect_review_readiness.py"

[[ -f "${skill_root}/SKILL.md" ]]
[[ -f "${skill_root}/adl-skill.yaml" ]]
[[ -f "${skill_root}/agents/openai.yaml" ]]
[[ -f "${skill_root}/references/review-readiness-cleanup-playbook.md" ]]
[[ -f "${skill_root}/references/output-contract.md" ]]
[[ -x "${python_script}" ]]
[[ -f "${skills_root}/docs/REVIEW_READINESS_CLEANUP_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "review-readiness-cleanup"' "${skill_root}/adl-skill.yaml"
grep -Fq 'id: "review_readiness_cleanup.v1"' "${skill_root}/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REVIEW_READINESS_CLEANUP_SKILL_INPUT_SCHEMA.md"' "${skill_root}/adl-skill.yaml"
grep -Fq "stop_before_review_approval_must_be_true" "${skill_root}/adl-skill.yaml"
grep -Fq "readiness cleanup classifier" "${skill_root}/SKILL.md"
grep -Fq "review_approved: false" "${skill_root}/references/output-contract.md"
grep -Fq "Schema id: \`review_readiness_cleanup.v1\`" "${skills_root}/docs/REVIEW_READINESS_CLEANUP_SKILL_INPUT_SCHEMA.md"

review_root="${tmpdir}/review"
report_root="${tmpdir}/review-readiness-report"
mkdir -p "${review_root}"
cat >"${review_root}/REVIEW_PLAN.md" <<'MD'
# Review Plan

Status: TODO before final internal review.
This packet has one skipped surface because external review is intentionally absent.
MD
cat >"${review_root}/FINDING_REGISTER.md" <<'MD'
# Finding Register

Finding: P1 review-blocking traceability gap.
Follow-on: create a small docs cleanup issue after the review starts.
MD
cat >"${review_root}/DEMO_PROOF_REGISTER.md" <<'MD'
# Demo Proof Register

Demo proof coverage is linked here.
MD

python3 "${python_script}" \
  --review-root "${review_root}" \
  --out "${report_root}" \
  --run-id review-readiness-contract-test >/tmp/review-readiness.out

[[ -f "${report_root}/review_readiness_cleanup_report.json" ]]
[[ -f "${report_root}/review_readiness_cleanup_report.md" ]]
grep -Fq '"schema": "adl.review_readiness_cleanup_report.v1"' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"run_id": "review-readiness-contract-test"' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"status": "blocked"' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"safe_mechanical_cleanup": 1' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"blocker": 1' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"skipped": 1' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"follow_on_needed": 1' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"review_approved": false' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq '"mutated_repository": false' "${report_root}/review_readiness_cleanup_report.json"
grep -Fq "## Classification Counts" "${report_root}/review_readiness_cleanup_report.md"
grep -Fq "## Safe Mechanical Cleanup" "${report_root}/review_readiness_cleanup_report.md"
grep -Fq "## Blockers" "${report_root}/review_readiness_cleanup_report.md"
grep -Fq "review_approved: false" "${report_root}/review_readiness_cleanup_report.md"

missing_report="${tmpdir}/missing-report"
python3 "${python_script}" \
  --review-root "${tmpdir}/does-not-exist" \
  --out "${missing_report}" \
  --run-id review-readiness-missing-test >/tmp/review-readiness-missing.out
grep -Fq '"status": "skipped"' "${missing_report}/review_readiness_cleanup_report.json"
grep -Fq '"skipped": 1' "${missing_report}/review_readiness_cleanup_report.json"

if grep -R "${tmpdir}" "${report_root}" "${missing_report}" >/dev/null; then
  echo "review readiness artifacts should not leak absolute temp paths" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skill_root}/SKILL.md"

echo "PASS test_review_readiness_cleanup_skill_contracts"

