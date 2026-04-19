#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
skills_root="${repo_root}/adl/tools/skills"
tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

[[ -f "${skills_root}/finding-to-issue-planner/SKILL.md" ]]
[[ -f "${skills_root}/finding-to-issue-planner/adl-skill.yaml" ]]
[[ -f "${skills_root}/finding-to-issue-planner/agents/openai.yaml" ]]
[[ -f "${skills_root}/finding-to-issue-planner/references/output-contract.md" ]]
[[ -x "${skills_root}/finding-to-issue-planner/scripts/plan_review_issues.py" ]]
[[ -f "${skills_root}/docs/FINDING_TO_ISSUE_PLANNER_SKILL_INPUT_SCHEMA.md" ]]

grep -Fq 'id: "finding-to-issue-planner"' "${skills_root}/finding-to-issue-planner/adl-skill.yaml"
grep -Fq 'id: "finding_to_issue_planner.v1"' "${skills_root}/finding-to-issue-planner/adl-skill.yaml"
grep -Fq 'reference_doc: "../docs/FINDING_TO_ISSUE_PLANNER_SKILL_INPUT_SCHEMA.md"' "${skills_root}/finding-to-issue-planner/adl-skill.yaml"
grep -Fq "tracker_creation_allowed_must_be_false" "${skills_root}/finding-to-issue-planner/adl-skill.yaml"
grep -Fq "Convert CodeBuddy review findings into grouped, reviewable issue candidates" "${skills_root}/finding-to-issue-planner/SKILL.md"
grep -Fq "Do not create issues, PRs, remediation branches, tests, or tracker items." "${skills_root}/finding-to-issue-planner/references/output-contract.md"
grep -Fq "Schema id: \`finding_to_issue_planner.v1\`" "${skills_root}/docs/FINDING_TO_ISSUE_PLANNER_SKILL_INPUT_SCHEMA.md"
grep -Fq "finding-to-issue-planner" "${skills_root}/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md"

review_path="${tmpdir}/review.md"
out_root="${tmpdir}/issue-plan"
cat >"${review_path}" <<'MARKDOWN'
# Code Review: example

## Findings

### Finding C-001: [P1] Unsafe root checkout mutation path

- Role: code
- Confidence: high
- Affected path or artifact: adl/tools/pr.sh
- Trigger scenario: execution runs from root main with tracked edits.
- Evidence: the review packet shows the helper accepts mutation before worktree binding.
- User/customer impact: operators can accidentally write tracked implementation changes on main.
- Recommended action: reject root-main tracked mutations and force issue worktree execution.
- Validation or proof gap: add a focused shell-contract test.
- Related findings: S-002

### Finding S-002: [P2] Unsafe root checkout mutation path

- Role: security
- Confidence: medium
- Affected path or artifact: adl/tools/pr.sh
- Trigger scenario: unsafe state can hide local drift.
- Evidence: the security review found the same mutation boundary.
- User/customer impact: local drift can reach review artifacts.
- Recommended action: preserve the root checkout safety gate.
MARKDOWN

python3 "${skills_root}/finding-to-issue-planner/scripts/plan_review_issues.py" \
  "${review_path}" --out "${out_root}" --tracker github >/tmp/finding-to-issue-planner.out

[[ -f "${out_root}/issue_candidates.json" ]]
[[ -f "${out_root}/issue_candidates.md" ]]
grep -Fq '"schema": "codebuddy.finding_to_issue_planner.v1"' "${out_root}/issue_candidates.json"
grep -Fq '"candidate_count": 1' "${out_root}/issue_candidates.json"
grep -Fq '"severity": "P1"' "${out_root}/issue_candidates.json"
grep -Fq '"source_finding_ids": [' "${out_root}/issue_candidates.json"
grep -Fq '"C-001"' "${out_root}/issue_candidates.json"
grep -Fq '"S-002"' "${out_root}/issue_candidates.json"
grep -Fq "Human approval is required before tracker mutation." "${out_root}/issue_candidates.md"
grep -Fq "No issues, PRs, tests, or remediation branches were created" "${out_root}/issue_candidates.md"
if grep -R "${tmpdir}" "${out_root}" >/dev/null; then
  echo "issue planning artifacts should not leak absolute temp paths" >&2
  exit 1
fi

packet_root="${tmpdir}/packet"
packet_out="${tmpdir}/packet-issue-plan"
mkdir -p "${packet_root}/specialist_reviews"
cat >"${packet_root}/specialist_reviews/code.md" <<'MARKDOWN'
# Code Review

## Findings

### Finding C-001: [P1] Duplicate raw packet finding

- Role: code
- Affected path or artifact: adl/tools/report.sh
- Evidence: raw specialist evidence.
- Impact: duplicate raw finding.
- Recommended action: use synthesis.
MARKDOWN
cat >"${packet_root}/specialist_reviews/security.md" <<'MARKDOWN'
# Security Review

## Findings

### Finding S-002: [P2] Duplicate raw packet finding

- Role: security
- Affected path or artifact: adl/tools/report.sh
- Evidence: raw specialist evidence.
- Impact: duplicate raw finding.
- Recommended action: use synthesis.
MARKDOWN
cat >"${packet_root}/specialist_reviews/synthesis.md" <<'MARKDOWN'
# Review Synthesis

## Findings

### Finding SYN-001: [P1] Synthesis grouped packet finding

- Role: synthesis
- Confidence: high
- Affected path or artifact: adl/tools/report.sh
- Evidence: synthesis groups C-001 and S-002 into one remediation bucket.
- Impact: downstream issue planning should not create one issue per raw specialist finding.
- Recommended action: create one bounded issue from the synthesis bucket.
- Validation or proof gap: rerun finding-to-issue planner contract tests.
- Related findings: C-001, S-002
MARKDOWN

python3 "${skills_root}/finding-to-issue-planner/scripts/plan_review_issues.py" \
  "${packet_root}" --out "${packet_out}" --tracker github >/tmp/finding-to-issue-planner-packet.out
grep -Fq '"candidate_count": 1' "${packet_out}/issue_candidates.json"
grep -Fq '"SYN-001"' "${packet_out}/issue_candidates.json"
if grep -Fq '"C-001"' "${packet_out}/issue_candidates.json" || grep -Fq '"S-002"' "${packet_out}/issue_candidates.json"; then
  echo "packet issue planning should prefer synthesis over duplicate specialist findings" >&2
  exit 1
fi

bash "${repo_root}/adl/tools/validate_skill_frontmatter.sh" \
  "${skills_root}/finding-to-issue-planner/SKILL.md"

echo "PASS test_finding_to_issue_planner_skill_contracts"
