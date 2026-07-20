# Structured Task Prompt

Template: 1.0.0

Issue: 5544

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair review-truth and external-review gate artifacts only; do not start external review or approve release readiness.

## Deliverables

- Refreshed sprint review register entries
- Refreshed WP-19 external-review handoff status
- Retained #5544 live-state evidence packet
- Typed lifecycle records and validation evidence

## Acceptance

1. AC-1: The canonical v0.91.7 sprint review register reflects current live GitHub and C-SDLC truth for WP-14 through WP-18
2. AC-2: #5404, #5413, #5527, #5408, and active closeout/audit PR truth are represented without premature release-readiness claims
3. AC-3: The WP-19 external-review handoff is explicitly blocked, ready, or conditionally ready based on current P1/P2 remediation state
4. AC-4: Existing-owner findings are routed without duplicate ownership
5. AC-5: Retained validation and live-state evidence support every refreshed state claim

## Dependencies

- #4645 internal review packet / PR #5543
- #4647 WP-20 remediation owner
- #5408 WP-07 terminal blocker
- #5527 terminal SOR artifact repair

## Inputs

- docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
- docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
- docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md
- docs/reviews/v0.91.7/internal-review-4645/FINDINGS_REGISTER.md
- .csdlc/issues/4644/index.json
- .csdlc/issues/5404/index.json
- .csdlc/issues/5413/index.json

## Non Goals

- Do not close #5408 or #5527
- Do not perform runtime, provider, or security code remediation
- Do not approve v0.91.7 release readiness
- Do not start WP-19 external review
- Do not use AWS
