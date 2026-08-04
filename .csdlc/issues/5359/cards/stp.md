# Structured Task Prompt

Template: 1.0.0

Issue: 5359

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare lifecycle packet only; planning review is future execution work.

## Deliverables

- generated six-card preparation packet
- concise design and diagram
- focused typed doctor validation

## Acceptance

1. AC-1: #5359 preparation is grounded in the live issue body, current origin/main, and checked-in v0.91.8 release-tail routing.
2. AC-2: future execution is blocked until WP-21A #5355 is closed by a merged PR and the observed #5355 merge SHA is ancestral to the exact #5359 execution base.
3. AC-3: future review inventory covers the WP wave, release-tail sequence, next-milestone handoff, activation map, canonical inventory, V092 handoff feature, and the WP-21/WP-21A output packets.
4. AC-4: the WP-22 output packet classifies blockers, stale assumptions, overclaims, explicit non-claims, and whether WP-23 #5348 may start.
5. AC-5: execution and preparation avoid main, #5357, PR #5805, #5804, version:v0.92 issues, product implementation, AWS, publication, merge, and closeout surfaces unless separately authorized.
6. AC-6: validation separates runnable preparation hygiene from deferred future execution proof; skipped, pending, or missing predecessor evidence is a blocker or non-pass.
7. AC-7: this preparation branch may be committed and pushed, but no WP execution, review publication, PR creation, or closeout occurs in the preparation lane.

## Dependencies

- WP-21A #5355 live merged into origin/main
- #5355 observed merge SHA ancestral to exact #5359 execution base

## Inputs

- docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
- docs/milestones/v0.91.8/features/V092_HANDOFF_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- issue #5359

## Non Goals

- planning review during preparation
- v0.92 opening
- PR publication
- receipt-gated execution
