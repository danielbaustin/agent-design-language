#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/review-quality-evaluator/SKILL.md" ]]
[[ -f "${skills_root}/review-quality-evaluator/adl-skill.yaml" ]]
[[ -f "${skills_root}/review-quality-evaluator/agents/openai.yaml" ]]
[[ -f "${skills_root}/review-quality-evaluator/references/evaluation-playbook.md" ]]
[[ -f "${skills_root}/review-quality-evaluator/references/output-contract.md" ]]
[[ -x "${skills_root}/review-quality-evaluator/scripts/evaluate_review_quality.py" ]]
[[ -f "${skills_root}/docs/REVIEW_QUALITY_EVALUATOR_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "review-quality-evaluator"' "${skills_root}/review-quality-evaluator/adl-skill.yaml"
grep -Fq 'id: "review_quality_evaluator.v1"' "${skills_root}/review-quality-evaluator/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/REVIEW_QUALITY_EVALUATOR_SKILL_INPUT_SCHEMA.md"' "${skills_root}/review-quality-evaluator/adl-skill.yaml"
grep -Fq "policy.reject_unsupported_claims_must_be_true" "${skills_root}/review-quality-evaluator/adl-skill.yaml"
grep -Fq "Evaluate a CodeBuddy review packet" "${skills_root}/review-quality-evaluator/SKILL.md"
grep -Fq "Treat unsupported approval, compliance, publication, merge-readiness, or" "${skills_root}/review-quality-evaluator/references/output-contract.md"
grep -Fq "Schema id: \`review_quality_evaluator.v1\`" "${skills_root}/docs/REVIEW_QUALITY_EVALUATOR_SKILL_INPUT_SCHEMA.md"
grep -Fq "review-quality-evaluator" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

packet_root="${tmpdir}/packet"
quality_root="${tmpdir}/quality-evaluation"
mkdir -p "${packet_root}/specialist_reviews" "${packet_root}/product-report"
cat >"${packet_root}/run_manifest.json" <<'JSON'
{
  "schema": "codebuddy.repo_packet.run_manifest.v1",
  "run_id": "review-quality-contract-test",
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

- src
- docs

## Excluded paths

- target

## Non-reviewed surfaces

- generated artifacts

## Assumptions

- Review is bounded to release-critical surfaces.
MD
cat >"${packet_root}/specialist_reviews/code.md" <<'MD'
# Code Review

## Findings

### Finding C-001: [P1] Missing report quality gate

- Role: code
- Confidence: high
- Affected path or artifact: adl/tools/report.sh
- Trigger scenario: a report can be produced without a quality-gate artifact.
- Evidence: the packet contains a final report but no quality evaluation.
- User/customer impact: customers may receive a weak report that hides review gaps.
- Recommended action: require a quality evaluation artifact before publication.
- Validation or proof gap: add a focused contract test for the quality gate.
- Related findings: none
MD
cat >"${packet_root}/specialist_reviews/security.md" <<'MD'
# Security Review

## Findings

### Finding S-001: [P2] Redaction gate must remain visible

- Role: security
- Confidence: medium
- Affected path or artifact: redaction_report.md
- Trigger scenario: customer-private output is prepared without visible redaction status.
- Evidence: publication intent is customer-private and redaction is required.
- User/customer impact: private paths or source excerpts could leak.
- Recommended action: require redaction status in the quality gate.
- Validation or proof gap: verify customer-private output fails without redaction.
MD
cat >"${packet_root}/specialist_reviews/dependencies.md" <<'MD'
# Dependency Review

## Findings

No dependency findings.
MD
cat >"${packet_root}/final_report.md" <<'MD'
# CodeBuddy Review Report: example-repo

## Executive Summary

- Overall risk: high

## Review Scope

- Repository: example-repo

## Top Findings

### Finding C-001: [P1] Missing report quality gate

- Source role: code
- Confidence: high
- Evidence: the packet contains a final report but no quality evaluation.
- Impact: customers may receive a weak report that hides review gaps.
- Recommended action: require a quality evaluation artifact before publication.
- Validation gap: add a focused contract test for the quality gate.

## Architecture Summary

- No architecture-specific findings.

## Security And Privacy Notes

- Redaction result: pass.

## Test Recommendations

- Linked finding: C-001.

## Remediation Sequence

1. Add quality-gate contract.

## Residual Risks

- Specialist coverage remains bounded.
MD
cat >"${packet_root}/redaction_report.md" <<'MD'
# Redaction Report

- Final publication status: pass.
MD

python3 "${skills_root}/review-quality-evaluator/scripts/evaluate_review_quality.py" \
  "${packet_root}" --out "${quality_root}" --publication-intent customer_private \
  --required-role code --required-role security --required-role dependencies >/tmp/review-quality-evaluator.out
[[ -f "${quality_root}/review_quality_evaluation.json" ]]
[[ -f "${quality_root}/review_quality_evaluation.md" ]]
grep -Fq '"schema": "codebuddy.review_quality_evaluation.v1"' "${quality_root}/review_quality_evaluation.json"
grep -Fq '"repo_name": "example-repo"' "${quality_root}/review_quality_evaluation.json"
grep -Fq '"status": "pass"' "${quality_root}/review_quality_evaluation.json"
grep -Fq '"publication_intent": "customer_private"' "${quality_root}/review_quality_evaluation.json"
grep -Fq '"published_by_skill": false' "${quality_root}/review_quality_evaluation.json"
grep -Fq '"approval_claimed": false' "${quality_root}/review_quality_evaluation.json"
grep -Fq '"dependencies"' "${quality_root}/review_quality_evaluation.json"
python3 - "${quality_root}/review_quality_evaluation.json" <<'PY'
import json
import sys
evaluation = json.load(open(sys.argv[1], encoding="utf-8"))
assert "dependencies" in evaluation["specialist_coverage"]["present_roles"]
assert "dependencies" not in evaluation["specialist_coverage"]["missing_roles"]
assert not [
    warning for warning in evaluation["warnings"]
    if warning.get("check") == "duplication"
], evaluation["warnings"]
PY
grep -Fq "## Quality Gate Summary" "${quality_root}/review_quality_evaluation.md"
grep -Fq "## Scorecard" "${quality_root}/review_quality_evaluation.md"
grep -Fq "## Publication Boundary" "${quality_root}/review_quality_evaluation.md"

rm "${packet_root}/redaction_report.md"
python3 "${skills_root}/review-quality-evaluator/scripts/evaluate_review_quality.py" \
  "${packet_root}" --out "${quality_root}/fail" --publication-intent customer_private \
  --required-role code --required-role security --required-role dependencies >/tmp/review-quality-evaluator-fail.out
grep -Fq '"status": "fail"' "${quality_root}/fail/review_quality_evaluation.json"
grep -Fq '"check": "redaction"' "${quality_root}/fail/review_quality_evaluation.json"

if grep -R "${tmpdir}" "${quality_root}" >/dev/null; then
  echo "review quality artifacts should not leak absolute temp paths" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/review-quality-evaluator/SKILL.md"

echo "PASS test_review_quality_evaluator_skill_contracts"
