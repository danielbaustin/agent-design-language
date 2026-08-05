# Structured Task Prompt

Template: 1.0.0

Issue: 5363

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare lifecycle packet only; remediation is future execution work.

## Deliverables

- generated six-card preparation packet
- concise design and diagram
- focused typed doctor validation

## Acceptance

1. AC-1: #5363 remains a preparation-only C-SDLC v2 packet with design, diagram, and six cards regenerated or updated through typed v2 routes.
2. AC-2: future WP-20 execution is blocked until WP-19 #5357 has a live merged revision on origin/main and that merge SHA is an ancestor of the exact #5363 execution base.
3. AC-3: completed child #5548 is verified as CLOSED/COMPLETED and its causal merged PR #5598 commit aac8eaa7dffaa904ed1dfb0ec17fbf667c1ef9f0 is ancestral to current origin/main before execution.
4. AC-4: completed child #5558 is verified as CLOSED/COMPLETED and closing PR commits c34f0c9412495039a6374f7ce88fa39e34bb5042 (#5749) and a5df18f19a4c651eb6594e5690e294c7b7929261 (#5769) are ancestral to current origin/main before execution.
5. AC-5: future remediation inventories accepted WP-18/WP-19 findings, fixes only accepted scope, and records unresolved, unsupported, or separately owned items as blockers or routed non-goals.
6. AC-6: future preflight runs focused checks for each accepted fix class plus the integrated v0.91.8 release preflight, then obtains exact-revision review before any publication or WP-21 admission.

## Dependencies

- WP-19 #5357 live merged into origin/main
- #5357 observed merge SHA ancestral to exact #5363 execution base

## Inputs

- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- issue #5363

## Non Goals

- remediation implementation
- review reruns
- PR publication
- receipt-gated execution
