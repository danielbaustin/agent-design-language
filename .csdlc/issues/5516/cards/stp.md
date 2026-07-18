# Structured Task Prompt

Template: 1.0.0

Issue: 5516

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair retained design metadata and terminal documentation without changing runtime source.

## Deliverables

- Corrected #5494 retained design
- Corrected #5494 retained architecture diagram
- Regenerated typed design and diagram digests
- Reviewed terminal reconciliation packet

## Acceptance

1. AC1: retained #5494 design no longer claims Runtime v2-only or no Runtime v3 changes
2. AC2: retained diagram shows Runtime v3 soak, CSM production integration, and weather ownership
3. AC3: typed terminal design repair succeeds and doctor passes
4. AC4: docs and live issue/PR state agree
5. AC5: exact review is clean before publication

## Dependencies

- Merged PR #5504
- Closed-out #5494 terminal receipt
- Typed csdlc-closeout repair-design operation

## Inputs

- Final terminal review finding on #5494 retained design
- docs/review-fixes/runtime/WP07A_REARCHITECTURE_REPAIR_5409.md
- .csdlc/issues/5494/retained/design.md

## Non Goals

- Runtime source changes
- Runtime v2 source work
- Weather-service implementation
- AWS execution
