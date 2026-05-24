---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0-91-3-wp-02-cognitive-transition-schema-review-prompt"
issue: 3200
task_id: "issue-3200"
version: "v0.91.3"
title: "[v0.91.3][WP-02][docs/tools] Cognitive Transition schema"
branch: "codex/3200-v0-91-3-wp-02-cognitive-transition-schema"
status: "approved"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3200"
  - kind: "stp"
    ref: ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/spp.md"
  - kind: "sor"
    ref: ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
review_results_exception: "Local bounded pre-PR review was completed without a review subagent in this turn. The card records that truth explicitly rather than inventing an absent subagent review."
scope_basis:
  - ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/stp.md"
  - ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sip.md"
  - ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/spp.md"
in_scope_surfaces:
  - "tracked WP-02 Rust schema, fixture, and doc changes"
  - "local ignored WP-02 issue-card truth"
evidence_policy:
  - "Use repository evidence, focused validation output, and linked issue-bundle artifacts only."
validation_inputs:
  - "Issue-local proofs recorded in the SOR."
  - "Focused Rust/schema/fixture validation results from the bound WP-02 run."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen issue scope beyond the first manifest schema, fixtures, and proof-doc surfaces."
  - "Do not merge, publish, or close the issue."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
  - "Refuse approving behavior outside the recorded issue scope."
follow_up_routing:
  - "Route broader transition DAG, shard-plan, signed-trace, or ObsMem work to downstream v0.91.3 and v0.91.4 issues instead of widening WP-02."
non_claims:
  - "This prompt does not claim the first manifest schema is the full C-SDLC implementation."
  - "This prompt does not guarantee review quality by itself."
policy_refs:
  - ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/stp.md"
  - ".adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sip.md"
notes: "SRP normalized into the modern Structured Review Prompt shape before publication."
---

# Structured Review Prompt

## Review Summary

Use this prompt to govern the bounded pre-PR review for `WP-02`. The review
focus is schema truthfulness: the new manifest surface, fixtures, and docs
must prove a real first transition-manifest slice without overclaiming later
C-SDLC substrate work.

## Scope Basis

- `.adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/stp.md`
- `.adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/sip.md`
- `.adl/v0.91.3/tasks/issue-3200__v0-91-3-wp-02-cognitive-transition-schema/spp.md`

## In-Scope Surfaces

- Tracked WP-02 Rust schema, fixture, and doc changes.
- Local ignored WP-02 issue-card truth.

## Evidence Rules

- Use repository evidence, focused validation output, and linked issue-bundle artifacts only.

## Validation Inputs

- Issue-local proofs recorded in the SOR.
- Focused Rust/schema/fixture validation results from the bound WP-02 run.

## Review Results

### Findings

- No actionable findings were identified in the bounded pre-PR review of the
  WP-02 schema module, tracked fixtures, and milestone docs.

### Finding Dispositions

- No fix-required findings.

### Recommended Outcome

- `pass`

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen issue scope beyond the first manifest schema, fixtures, and proof-doc surfaces.
- Do not merge, publish, or close the issue.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.
- Refuse approving behavior outside the recorded issue scope.

## Follow-up Routing

- Route broader transition DAG, shard-plan, signed-trace, or ObsMem work to downstream v0.91.3 and v0.91.4 issues instead of widening WP-02.

## Non-Claims

- This prompt does not claim the first manifest schema is the full C-SDLC implementation.
- This prompt does not guarantee review quality by itself.

## Residual Risks

- WP-02 establishes a real first schema slice, but downstream issues still need to connect it to transition DAG, shard-plan, signed-trace, and ObsMem surfaces.
- No review subagent was used in this turn; that process exception remains visible instead of being silently erased.

## Notes

SRP normalized into the modern Structured Review Prompt shape and updated with
the local bounded pre-PR review result before publication.
