---
schema_version: "0.1"
artifact_type: "structured_review_prompt"
name: "v0-91-3-wp-03-card-lifecycle-demo-review-prompt"
issue: 3201
task_id: "issue-3201"
version: "v0.91.3"
title: "[v0.91.3][WP-03][tools] Card lifecycle integration"
branch: "codex/3201-v0-91-3-wp-03-card-lifecycle-integration"
status: "approved"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/3201"
  - kind: "stp"
    ref: "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  - kind: "sip"
    ref: "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
  - kind: "spp"
    ref: "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
  - kind: "sor"
    ref: "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sor.md"
review_mode: "pre_pr_independent_review"
timing: "before_pr_open"
review_results:
  findings_status: "no_findings"
  recommended_outcome: "pass"
scope_basis:
  - "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/stp.md"
  - "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sip.md"
  - "workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/spp.md"
in_scope_surfaces:
  - "tracked public C-SDLC card bundle"
  - "focused validator and doctor proof for the bundle"
evidence_policy:
  - "Use repository evidence, focused validation output, and tracked bundle artifacts only."
validation_inputs:
  - "Validator and doctor proof recorded in the paired SOR."
allowed_dispositions:
  - "PASS"
  - "BLOCK"
  - "NEEDS_FOLLOWUP"
reviewer_constraints:
  - "Do not widen WP-03 into default-operation rollout."
  - "Do not collapse review truth into output truth."
refusal_policy:
  - "Refuse claims that are unsupported by repository evidence."
follow_up_routing:
  - "Route later default-operation lifecycle hardening to v0.91.4 follow-on work."
non_claims:
  - "This prompt does not claim full C-SDLC completion."
  - "This prompt does not replace local active issue bundles."
policy_refs:
  - "docs/cognitive-sdlc/card-lifecycle.md"
  - "docs/tooling/structured-prompt-contracts.md"
notes: "Tracked public SRP proof surface for WP-03."
---

# Structured Review Prompt

## Review Summary

Review the tracked public `WP-03` bundle as proof that the canonical
`SIP -> STP -> SPP -> SRP -> SOR` lifecycle is validator-backed and
doctor-backed, not just documented.

## Scope Basis

- `workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/stp.md`
- `workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/sip.md`
- `workflow/c-sdlc/v0.91.3/issues/issue-3201-card-lifecycle-demo/cards/spp.md`

## In-Scope Surfaces

- Tracked public C-SDLC card bundle.
- Focused validator and doctor proof for the bundle.

## Evidence Rules

- Use repository evidence, focused validation output, and tracked bundle artifacts only.

## Validation Inputs

- Validator and doctor proof recorded in the paired SOR.

## Review Results

### Findings

- No actionable findings were identified in the tracked public card bundle or
  the focused validator/doctor proof.

### Finding Dispositions

- No fix-required findings.

### Recommended Outcome

- `pass`

## Allowed Dispositions

- PASS
- BLOCK
- NEEDS_FOLLOWUP

## Reviewer Constraints

- Do not widen WP-03 into default-operation rollout.
- Do not collapse review truth into output truth.

## Refusal Policy

- Refuse claims that are unsupported by repository evidence.

## Follow-up Routing

- Route later default-operation lifecycle hardening to `v0.91.4` follow-on work.

## Non-Claims

- This prompt does not claim full C-SDLC completion.
- This prompt does not replace local active issue bundles.

## Residual Risks

- The tracked bundle proves the first slice only; later issues still need to
  connect this lifecycle proof to broader transition substrate and memory work.

## Notes

Tracked public SRP proof surface for `WP-03`.
