---
name: review-readiness-cleanup
description: Inspect review packets, review plans, finding registers, demo or proof registers, and milestone review docs for structural readiness, classifying safe cleanup, blockers, skipped surfaces, and follow-on needs without remediating findings, publishing reports, or approving review readiness.
---

# Review Readiness Cleanup

Inspect one review packet or milestone review surface before an internal,
external, or CodeBuddy-style review cycle starts. This skill reduces avoidable
review friction from stale markers, missing metadata, unclear blockers, and
packet-structure drift.

It is a readiness cleanup classifier, not a qualitative reviewer and not an
approval authority.

## Quick Start

1. Confirm the review root and intended review cycle.
2. Confirm the cleanup policy stops before remediation, publication, and review
   approval.
3. Run the deterministic helper when local filesystem access is available:
   - `scripts/inspect_review_readiness.py --review-root <path> --out <artifact-root> --run-id <run_id>`
4. Review the Markdown and JSON artifacts.
5. Route actionable results to the appropriate owner. Do not rewrite findings,
   alter severity, publish reports, or approve review readiness from this skill.

## Required Inputs

At minimum, gather:

- `mode`
- `review_root`
- `target`
- `policy`

Supported modes:

- `inspect_review_packet`
- `inspect_milestone_review`
- `refresh_readiness_cleanup`

Useful policy fields:

- `write_cleanup_artifact`
- `allow_safe_mechanical_cleanup`
- `require_finding_register`
- `require_demo_or_proof_register`
- `stop_before_remediation`
- `stop_before_publication`
- `stop_before_review_approval`

If the review root is missing or intentionally not in scope, return `skipped`
with the reason visible.

## Classification Model

Classify findings as:

- `safe_mechanical_cleanup`: stale placeholders, unchecked metadata, empty
  packet sections, or formatting drift that can be repaired without changing
  substantive findings.
- `blocker`: missing required review surfaces, explicit blocker markers,
  unresolved high-priority review state, or evidence drift that would make the
  review misleading.
- `skipped`: a review surface was intentionally absent, unavailable, or out of
  scope for this cycle.
- `follow_on_needed`: useful cleanup or process improvement that should not
  block the current review but should be queued.

If evidence is insufficient, classify as `blocker` or `skipped`; do not infer
readiness from absence.

## Output

Write Markdown and JSON artifacts when an output root is available.

Default artifact root:

```text
.adl/reviews/review-readiness-cleanup/<run_id>/
```

Required artifacts:

- `review_readiness_cleanup_report.md`
- `review_readiness_cleanup_report.json`

Use the detailed contract in `references/output-contract.md`.

## Stop Boundary

This skill must not:

- run a full internal or external review
- remediate review findings
- rewrite finding severity, disagreement, evidence, or conclusions
- publish customer-facing reports
- approve review readiness without evidence
- create issues or PRs without explicit operator approval

Handoff candidates:

- `documentation-specialist` for approved mechanical doc cleanup.
- `gap-analysis` when expected review scope needs comparison against evidence.
- `finding-to-issue-planner` when cleanup or blockers should become issue
  candidates.
- `review-quality-evaluator` when the review packet itself needs a quality gate.

