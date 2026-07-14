# Structured Task Prompt

Template: 1.0.0

Issue: 5306

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Execute only exact approved manifest slices and recompute proof after every slice.

## Deliverables

- Approved exact-revision deletion manifest
- Bounded deletion PR slices
- Retained-surface register
- Post-slice v2 and LoC/test proof

## Acceptance

1. D1 eligibility is current and true before binding
2. Explicit operator deletion approval is recorded
3. Rollback and importer surfaces remain
4. Every slice is independently reviewed and green
5. Removal is measured against the 90 percent target; retained useful code is reviewable and explicitly justified

## Dependencies

- #5305 merged
- Current eligible D1 decision
- Explicit operator deletion approval
- #5295 umbrella remains open

## Inputs

- docs/architecture/csdlc-v2/gate10d1/ELIGIBILITY_EVIDENCE.json
- csdlc-v2/operator/eligibility-request.json
- docs/architecture/csdlc-v2/CSDLC_V1_BASELINE_AND_V2_BUDGETS.md

## Non Goals

- Early rollback sunset
- Early importer sunset
- Unrelated ADL or Runtime cleanup
