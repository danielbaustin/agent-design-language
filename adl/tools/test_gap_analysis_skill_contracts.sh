#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/gap-analysis/SKILL.md" ]]
[[ -f "${skills_root}/gap-analysis/adl-skill.yaml" ]]
[[ -f "${skills_root}/gap-analysis/agents/openai.yaml" ]]
[[ -f "${skills_root}/gap-analysis/references/gap-analysis-playbook.md" ]]
[[ -f "${skills_root}/gap-analysis/references/output-contract.md" ]]
[[ -x "${skills_root}/gap-analysis/scripts/analyze_gaps.py" ]]
[[ -f "${skills_root}/docs/GAP_ANALYSIS_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "gap-analysis"' "${skills_root}/gap-analysis/adl-skill.yaml"
grep -Fq 'id: "gap_analysis.v1"' "${skills_root}/gap-analysis/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/GAP_ANALYSIS_SKILL_INPUT_SCHEMA.md"' "${skills_root}/gap-analysis/adl-skill.yaml"
grep -Fq "expected_baseline_must_be_explicit" "${skills_root}/gap-analysis/adl-skill.yaml"
grep -Fq "Compare an explicit expected baseline" "${skills_root}/gap-analysis/SKILL.md"
grep -Fq "Distinguish missing evidence from proven failure." "${skills_root}/gap-analysis/references/output-contract.md"
grep -Fq "Schema id: \`gap_analysis.v1\`" "${skills_root}/docs/GAP_ANALYSIS_SKILL_INPUT_SCHEMA.md"

gap_root="${tmpdir}/gap"
report_root="${tmpdir}/gap-report"
mkdir -p "${gap_root}"
cat >"${gap_root}/gap_manifest.json" <<'JSON'
{
  "run_id": "gap-analysis-contract-test",
  "scope": "issue-2044",
  "mode": "compare_issue_to_implementation"
}
JSON
cat >"${gap_root}/expected_baseline.md" <<'MD'
# Expected Baseline

## Expected

- Create gap-analysis skill bundle with SKILL metadata and bounded stop boundary.
- Add validation contract tests for findings format and evidence guardrails.
- Document input schema for explicit expected baseline and observed evidence.
- Record closeout truth in output cards.
MD
cat >"${gap_root}/observed_evidence.md" <<'MD'
# Observed Evidence

- gap-analysis skill bundle includes SKILL metadata and a bounded stop boundary.
- input schema documents explicit expected baseline and observed evidence.
MD
cat >"${gap_root}/known_gaps.md" <<'MD'
# Known Gaps

## Gaps

- Validation contract tests for findings format and evidence guardrails are missing.
MD

python3 "${skills_root}/gap-analysis/scripts/analyze_gaps.py" \
  "${gap_root}" --out "${report_root}" >/tmp/gap-analysis.out
[[ -f "${report_root}/gap_analysis_report.json" ]]
[[ -f "${report_root}/gap_analysis_report.md" ]]
grep -Fq '"schema": "adl.gap_analysis_report.v1"' "${report_root}/gap_analysis_report.json"
grep -Fq '"run_id": "gap-analysis-contract-test"' "${report_root}/gap_analysis_report.json"
grep -Fq '"gap_type": "test_gap"' "${report_root}/gap_analysis_report.json"
grep -Fq '"created_issues": false' "${report_root}/gap_analysis_report.json"
grep -Fq '"mutated_repository": false' "${report_root}/gap_analysis_report.json"
grep -Fq "## Gap Analysis Summary" "${report_root}/gap_analysis_report.md"
grep -Fq "## Findings" "${report_root}/gap_analysis_report.md"
grep -Fq "Created issues: false." "${report_root}/gap_analysis_report.md"
grep -Fq "Mutated repository: false." "${report_root}/gap_analysis_report.md"

missing_root="${tmpdir}/missing-baseline"
mkdir -p "${missing_root}"
python3 "${skills_root}/gap-analysis/scripts/analyze_gaps.py" \
  "${missing_root}" --out "${tmpdir}/missing-report" >/tmp/gap-analysis-missing.out
grep -Fq '"status": "not_run"' "${tmpdir}/missing-report/gap_analysis_report.json"

if grep -R "${tmpdir}" "${report_root}" "${tmpdir}/missing-report" >/dev/null; then
  echo "gap analysis artifacts should not leak absolute temp paths" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/gap-analysis/SKILL.md"

echo "PASS test_gap_analysis_skill_contracts"
