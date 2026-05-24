---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0-91-3-wp-05-evidence-bundle-and-review-synthesis-review-prompt"
issue: 3203
task_id: "issue-3203"
version: "v0.91.3"
title: "[v0.91.3][WP-05][docs/tools] Evidence bundle and review synthesis"
branch: "codex/3203-v0-91-3-wp-05-evidence-bundle-and-review-synthesis"
status: "approved"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3203"
  - kind: "stp"
    ref: ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/spp.md"
  - kind: "sor"
    ref: ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
review_results_exception: "Local bounded pre-PR review was completed without a review subagent in this turn. The card records that truth explicitly rather than inventing an absent subagent review."
scope_basis:
  - ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/stp.md"
  - ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/sip.md"
  - ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/spp.md"
in_scope_surfaces:
  - "tracked WP-05 evidence-bundle packet, synthesis companion, validator/test lane, schema linkage proof, and milestone proof-doc updates"
  - "local ignored WP-05 issue-card truth"
evidence_policy:
  - "Use repository evidence, focused validation output, and linked issue-bundle artifacts only."
validation_inputs:
  - "Issue-local proofs recorded in the SOR."
  - "Focused evidence-bundle validator, contract-test, and schema linkage test results from the bound WP-05 run."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope beyond the tracked evidence-bundle packet, synthesis companion, validator/test lane, schema linkage proof, and milestone proof-doc updates."
  - "Do not merge, publish, or close the issue."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
  - "Refuse approving behavior outside the recorded issue scope."
follow_up_routing:
  - "Route governed merge gating, ObsMem handoff, and measured five-minute-sprint timing to downstream issues instead of widening WP-05."
non_claims:
  - "This prompt does not claim merge-readiness has already been proven."
  - "This prompt does not claim the five-minute-sprint timing demo already ran."
policy_refs:
  - ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/stp.md"
  - ".adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/sip.md"
notes: "SRP normalized into the modern Structured Review Prompt shape before publication."
---

# Structured Review Prompt

## Review Summary

Use this prompt to govern the bounded pre-PR review for `WP-05`. The review
focus is evidence convergence truthfulness: the tracked evidence-bundle packet,
review synthesis companion, validator/test proof, and schema linkage proof must
demonstrate a real bounded evidence surface without overclaiming merge gating,
ObsMem handoff, or measured first-proof timing that belong to later work
packages.

## Scope Basis

- `.adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/stp.md`
- `.adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/sip.md`
- `.adl/v0.91.3/tasks/issue-3203__v0-91-3-wp-05-evidence-bundle-and-review-synthesis/spp.md`

## In-Scope Surfaces

- Tracked `WP-05` evidence-bundle packet, synthesis companion, validator/test lane, schema linkage proof, and milestone proof-doc updates.
- Local ignored `WP-05` issue-card truth.

## Evidence Rules

- Use repository evidence, focused validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- Issue-local proofs recorded in the SOR.
- Focused evidence-bundle validator, contract-test, and schema linkage test results from the bound `WP-05` run.

## Review Results

### Findings

- No actionable findings were identified in the bounded pre-PR review of the
  tracked evidence-bundle packet, synthesis companion, validator/test lane,
  schema linkage proof, or milestone proof docs.

### Finding Dispositions

- No fix-required findings.

### Recommended Outcome

- `pass`

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope beyond the tracked evidence-bundle packet, synthesis companion, validator/test lane, schema linkage proof, and milestone proof-doc updates.
- Do not merge, publish, or close the issue.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.
- Refuse approving behavior outside the recorded issue scope.

## Follow-up Routing

- Route governed merge gating, ObsMem handoff, and measured five-minute-sprint timing to downstream issues instead of widening `WP-05`.

## Non-Claims

- This prompt does not claim merge-readiness has already been proven.
- This prompt does not claim the five-minute-sprint timing demo already ran.

## Residual Risks

- `WP-05` proves the evidence-bundle and review-synthesis shape only; later issues still need to connect it to merge gating, memory handoff, and measured timing.
- No review subagent was used in this turn; that process exception remains visible instead of being silently erased.

## Notes

SRP normalized into the modern Structured Review Prompt shape and updated with
the local bounded pre-PR review result before publication.
