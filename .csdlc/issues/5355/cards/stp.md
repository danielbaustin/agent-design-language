# Structured Task Prompt

Template: 1.0.0

Issue: 5355

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare lifecycle packet only; closeout-plan authoring is future execution work.

## Deliverables

- generated six-card preparation packet
- concise design and diagram
- focused typed doctor validation

## Acceptance

1. AC-1: six cards, design, and diagram are issue-specific and generated or regenerated through typed C-SDLC v2 routes
2. AC-2: later execution is blocked until WP-21 issue #5362 has a live merged PR and the observed merge commit is ancestral to refreshed origin/main and the exact #5355 execution base
3. AC-3: the future closeout-planning packet consumes reviewed WP-21 feature-list and v0.92 planning truth without mutating any version:v0.92 issue
4. AC-4: preparation remains issue-local and performs no implementation, #5357 remediation, PR publication, merge, closeout, AWS, or main-checkout mutation
5. AC-5: focused validation includes C-SDLC doctor, request-driven C-SDLC validation, predecessor live-state/ancestry checks, canonical docs/YAML checks, and diff hygiene

## Dependencies

- WP-21 #5362 live merged into origin/main
- #5362 observed merge SHA ancestral to exact #5355 execution base

## Inputs

- docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md
- docs/milestones/v0.91.8/NEXT_MILESTONE_HANDOFF_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- issue #5355

## Non Goals

- closeout-plan authoring during preparation
- birthday implementation
- PR publication
- receipt-gated execution
