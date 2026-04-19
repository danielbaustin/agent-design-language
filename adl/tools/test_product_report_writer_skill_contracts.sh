#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/product-report-writer/SKILL.md" ]]
[[ -f "${skills_root}/product-report-writer/adl-skill.yaml" ]]
[[ -f "${skills_root}/product-report-writer/agents/openai.yaml" ]]
[[ -f "${skills_root}/product-report-writer/references/report-writing-playbook.md" ]]
[[ -f "${skills_root}/product-report-writer/references/output-contract.md" ]]
[[ -x "${skills_root}/product-report-writer/scripts/write_product_report.py" ]]
[[ -f "${skills_root}/docs/PRODUCT_REPORT_WRITER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "product-report-writer"' "${skills_root}/product-report-writer/adl-skill.yaml"
grep -Fq 'id: "product_report_writer.v1"' "${skills_root}/product-report-writer/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/PRODUCT_REPORT_WRITER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/product-report-writer/adl-skill.yaml"
grep -Fq "policy.stop_before_publication_must_be_true" "${skills_root}/product-report-writer/adl-skill.yaml"
grep -Fq "Write a customer-grade CodeBuddy review report" "${skills_root}/product-report-writer/SKILL.md"
grep -Fq "Do not claim approval, compliance, merge-readiness, remediation completion" "${skills_root}/product-report-writer/references/output-contract.md"
grep -Fq "Schema id: \`product_report_writer.v1\`" "${skills_root}/docs/PRODUCT_REPORT_WRITER_SKILL_INPUT_SCHEMA.md"
grep -Fq "product-report-writer" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
report_root="${tmpdir}/product-report"
mkdir -p \
  "${packet_root}/specialist_reviews" \
  "${packet_root}/diagram-plan" \
  "${packet_root}/test-plan" \
  "${packet_root}/issue-plan" \
  "${packet_root}/quality-evaluation" \
  "${packet_root}/redaction-audit"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "product-report-contract-test",
  "repo_name": "example-repo",
  "repo_ref": "abc123",
  "review_mode": "release_review",
  "privacy_mode": "customer_private",
  "publication_allowed": false
}
JSON
cat >"${packet_root}/repo_scope.md" <<'MD'
# Repo Scope

## Included paths

- adl/src
- adl/tools

## Excluded paths

- target

## Non-reviewed surfaces

- generated artifacts

## Assumptions

- Review packet is bounded to release-critical surfaces.
MD
cat >"${packet_root}/specialist_reviews/code.md" <<'MD'
# Code Review

## Findings

### Finding C-001: [P1] Unsafe report overclaim path

- Role: code
- Confidence: high
- Affected path or artifact: adl/tools/report.sh
- Trigger scenario: report language claims approval without evidence.
- Evidence: the synthesis packet contains an approval-like phrase without a redaction or quality gate.
- User/customer impact: customers may believe remediation or approval happened when it did not.
- Recommended action: require explicit publication boundary language.
- Validation or proof gap: add report contract validation.
- Related findings: D-002
MD
cat >"${packet_root}/specialist_reviews/docs.md" <<'MD'
# Docs Review

## Findings

### Finding D-002: [P2] Unsafe report overclaim path

- Role: docs
- Confidence: medium
- Affected path or artifact: docs/review.md
- Evidence: docs wording can hide missing quality-gate status.
- User/customer impact: reviewer handoff may overstate certainty.
- Recommended action: surface missing gates in caveats.
MD
cat >"${packet_root}/specialist_reviews/synthesis.md" <<'MD'
# Review Synthesis

## Findings

### Finding SYN-001: [P1] Unsafe report overclaim path

- Role: synthesis
- Confidence: high
- Affected path or artifact: adl/tools/report.sh
- Evidence: specialist reviews C-001 and D-002 identify the same report-boundary failure.
- Impact: customers may believe remediation or approval happened when it did not.
- Recommended action: require explicit publication boundary language and surface supporting specialist evidence as related findings.
- Validation or proof gap: rerun product-report and quality-evaluation contract tests.
- Related findings: C-001, D-002
MD
cat >"${packet_root}/redaction-audit/redaction_report.md" <<'MD'
# Redaction Report

- Final publication status: partial.
MD
cat >"${packet_root}/quality-evaluation/review_quality_evaluation.md" <<'MD'
# Review Quality Evaluation
MD
cat >"${packet_root}/diagram-plan/repo_diagram_plan.md" <<'MD'
# Repo Diagram Plan
MD
cat >"${packet_root}/test-plan/review_to_test_plan.md" <<'MD'
# Review To Test Plan
MD
cat >"${packet_root}/issue-plan/issue_candidates.md" <<'MD'
# Issue Candidates
MD

python3 "${skills_root}/product-report-writer/scripts/write_product_report.py" \
  "${packet_root}" --out "${report_root}" --audience customer_private >/tmp/product-report-writer.out
[[ -f "${report_root}/codebuddy_product_report.json" ]]
[[ -f "${report_root}/codebuddy_product_report.md" ]]
grep -Fq '"schema": "codebuddy.product_report.v1"' "${report_root}/codebuddy_product_report.json"
grep -Fq '"repo_name": "example-repo"' "${report_root}/codebuddy_product_report.json"
grep -Fq '"severity": "P1"' "${report_root}/codebuddy_product_report.json"
grep -Fq '"redaction_report": true' "${report_root}/codebuddy_product_report.json"
grep -Fq '"quality_evaluation": true' "${report_root}/codebuddy_product_report.json"
grep -Fq '"diagram_manifest": true' "${report_root}/codebuddy_product_report.json"
grep -Fq '"test_plan": true' "${report_root}/codebuddy_product_report.json"
grep -Fq '"issue_plan": true' "${report_root}/codebuddy_product_report.json"
grep -Fq '"published_by_skill": false' "${report_root}/codebuddy_product_report.json"
grep -Fq '"approval_claimed": false' "${report_root}/codebuddy_product_report.json"
grep -Fq '"remediation_complete_claimed": false' "${report_root}/codebuddy_product_report.json"
grep -Fq "## Executive Summary" "${report_root}/codebuddy_product_report.md"
grep -Fq "## Top Findings" "${report_root}/codebuddy_product_report.md"
grep -Fq "## Residual Risks" "${report_root}/codebuddy_product_report.md"
grep -Fq "Approval claimed: false." "${report_root}/codebuddy_product_report.md"
grep -Fq "Compliance claimed: false." "${report_root}/codebuddy_product_report.md"
grep -Fq "Remediation complete claimed: false." "${report_root}/codebuddy_product_report.md"
grep -Fq "Redaction report present." "${report_root}/codebuddy_product_report.md"
if grep -Fq "Redaction report missing" "${report_root}/codebuddy_product_report.md"; then
  echo "product report should discover redaction-audit/redaction_report.md" >&2
  exit 1
fi
python3 - "${report_root}/codebuddy_product_report.json" <<'PY'
import json
import sys
report = json.load(open(sys.argv[1], encoding="utf-8"))
titles = [finding["title"] for finding in report["top_findings"]]
assert titles == ["Unsafe report overclaim path"], titles
assert report["top_findings"][0]["source_artifact"] == "specialist_reviews/synthesis.md"
assert report["remediation_sequence"].count(report["remediation_sequence"][0]) == 1
PY
if grep -R "${tmpdir}" "${report_root}" >/dev/null; then
  echo "product report artifacts should not leak absolute temp paths" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/product-report-writer/SKILL.md"

echo "PASS test_product_report_writer_skill_contracts"
