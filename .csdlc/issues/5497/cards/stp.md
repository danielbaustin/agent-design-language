# Structured Task Prompt

Template: 1.0.0

Issue: 5497

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Preparation and coordination only; no product implementation belongs to #5497.

## Deliverables

- validated six-card umbrella packet
- child order and authority diagram
- non-overlap and completion checklist

## Acceptance

1. AC-1: #5499, #5498, #5500, #5502, and #5501 retain disjoint issue ownership and execute in the canonical order
2. AC-2: WP-09 live merge and ancestry, not receipt presence, release #5499 preparation and execution
3. AC-3: #5500 and #5502 may execute in parallel after #5498 freezes observation and output contracts
4. AC-4: #5501 uses real tasks and disjoint writable shards; fixture-only evidence cannot satisfy the live proof
5. AC-5: #5497 owns no product path and cannot create tasks, mutate GitHub, merge, or close child issues
6. AC-6: Umbrella completion reports every child merge or explicit disposition and preserves receipts only as audit evidence

## Dependencies

- WP-09 merged into current origin/main
- #5499 then #5498
- #5500 and #5502 after #5498
- #5501 after all implementation children

## Inputs

- AGENTS.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/PARALLEL_EXECUTION_PLAN_v0.91.8.md
- issues #5497-#5502

## Non Goals

- product implementation
- a second lifecycle store or scheduler
- autonomous merge or closeout
- receipt-gated execution
- Runtime v2 or AWS work
