---
name: review-quality-evaluator
description: Evaluate CodeBuddy review packets and reports against the third-party-review quality bar for evidence quality, severity accuracy, actionability, duplication, unsupported claims, specialist coverage, template compliance, residual risk clarity, and publication safety before customer-facing use.
---

# Review Quality Evaluator

Evaluate a CodeBuddy review packet or product report before it becomes a
customer-facing artifact. This skill is a quality gate, not a specialist
reviewer, not a remediator, not a publisher, and not an approval authority.

Use this skill after packet building, specialist reviews, synthesis, redaction,
diagram review, test planning, issue planning, and report writing have produced
the artifacts that need to be judged.

## Quick Start

1. Confirm the evaluation source:
   - CodeBuddy packet root
   - product report artifact root
   - synthesis or final report
   - specialist artifact bundle
2. Confirm publication intent and required specialist roles.
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/evaluate_review_quality.py <packet-root> --out <artifact-root>`
4. Review the gate status, blocking reasons, warnings, and scorecard.
5. Stop before publication, approval claims, issue creation, PR creation,
   remediation, report rewriting, specialist review, or customer repository
   mutation.

## Required Inputs

At minimum, gather:

- `artifact_root` or `packet_root`
- `mode`
- `publication_intent`
- `policy`

Supported modes:

- `evaluate_packet`
- `evaluate_report`
- `evaluate_synthesis`
- `pre_publication_gate`

Useful policy fields:

- `publication_intent`
- `required_roles`
- `severity_floor`
- `require_redaction_status`
- `require_template_sections`
- `reject_unsupported_claims`
- `write_evaluation_artifact`
- `stop_before_publication`
- `stop_before_mutation`

If there is no concrete artifact root or report source, stop and report
`not_run`.

## Workflow

### 1. Establish Gate Boundary

Record:

- packet or report source
- repo name and ref, if present
- publication intent
- required specialist roles
- report template sections expected
- redaction status
- non-reviewed surfaces and assumptions

Do not upgrade publication readiness. This skill can only report whether the
quality gate passes, partially passes, fails, or could not run.

### 2. Evaluate Review Quality

Check:

- findings have evidence, impact, recommended action, and validation gap
- severity is justified by concrete impact
- findings are ordered by severity where possible
- duplicate findings are linked, merged, or explicitly preserved
- unsupported approval, compliance, publication, or remediation claims are absent
- specialist coverage is present or missing roles are clearly caveated
- report sections follow the CodeBuddy review template standard
- non-reviewed surfaces, caveats, disagreements, and residual risks are visible
- redaction and evidence-boundary artifacts exist before customer-facing output

### 3. Classify The Gate

Use these statuses:

- `pass`: the source satisfies the required quality checks for internal review.
- `partial`: the source is usable but warnings or missing non-blocking evidence
  remain.
- `fail`: one or more blockers should prevent customer-facing publication.
- `not_run`: no readable source was available.

Use blockers for missing evidence on findings, unsupported publication or
approval claims, missing redaction before customer-facing publication, unclear
scope, hidden residual risk, or missing required template sections.

Use warnings for missing optional specialist roles, weak actionability, duplicate
findings that need manual review, or incomplete validation gaps.

### 4. Stop Before Publication

This skill must not:

- publish, send, or approve a report
- claim compliance, merge-readiness, remediation completion, or security approval
- create issues, pull requests, tests, diagrams, ADRs, or fixes
- edit or rewrite customer reports
- edit customer repositories
- run specialist review lanes
- hide missing coverage or residual risk

Handoff candidates to:

- `product-report-writer` when the quality gate finds report structure gaps
- `redaction-and-evidence-auditor` when publication safety is missing or unsafe
- `repo-review-synthesis` when specialist disagreement or dedupe is unresolved
- `finding-to-issue-planner` for approved follow-through after human review

## Output

Write Markdown and JSON quality artifacts when an output root is available.

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/quality-evaluation/
```

Required artifacts:

- `review_quality_evaluation.md`
- `review_quality_evaluation.json`

Use the detailed contract in `references/output-contract.md`.

## Blocked States

Return `not_run` when the source is missing or unreadable.

Return `fail` when:

- findings lack evidence
- severity lacks impact justification
- scope or non-reviewed surfaces are unclear
- customer-facing publication lacks redaction status
- unsupported approval, compliance, merge-readiness, publication, or remediation
  claims appear
- required template sections are missing

Return `partial` when the review can be used internally but missing specialist
coverage, weak actionability, duplicated findings, or incomplete caveats need
human review before publication.
