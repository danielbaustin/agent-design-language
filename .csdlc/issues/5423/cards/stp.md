# Structured Task Prompt

Template: 1.0.0

Issue: 5423

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Reconcile #5036 remediation and any other terminal remediation evidence in one bounded canonical-register pass.

## Deliverables

- Truthful canonical register update
- Retained validation and independent review evidence

## Acceptance

1. #5403 terminal claim release is verified
2. The #5036 row reflects completed #5406/#5407 remediation
3. Additional rows change only with terminal retained evidence
4. Nonterminal remediation rows remain unchanged
5. The register patch passes focused validation and exact-revision review

## Dependencies

- #5403 terminal lifecycle reconciliation
- #5407 completed remediation

## Inputs

- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- docs/reviews/v0.91.7/remaining-sprints-5403
- .git/csdlc-v2/closeout

## Non Goals

- Product or runtime code changes
- Changes to other sessions' cards or worktrees
- Premature pass claims
