# Structured Task Prompt

Template: 1.0.0

Issue: 5862

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Schedule and reconcile exactly WP-04.01 through WP-04.16 without direct product implementation.

## Deliverables

- Validated live sixteen-child mapping
- Dependency-aware scheduling record
- Child terminal evidence matrix
- WP-04.16 integrated proof handoff to #5832

## Acceptance

1. Exactly sixteen prepared mapped children exist
2. Mapping and dependencies match #5821 and canonical wave
3. All preparation claims are null before scheduling
4. Each child executes only after dependencies and path claims pass
5. WP-04.16 proves real multi-node API/WSS, partition, fencing, migration, recovery, shutdown, and native platforms
6. Final reconciliation derives exact-head terminal evidence

## Dependencies

- #5821 architecture/security gate terminal
- #5820 WP-03 terminal
- WP-04.01 through WP-04.16 prepared

## Inputs

- .csdlc/prepared/issues/5821/design.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WBS_v0.92.md

## Non Goals

- Direct product implementation
- Child lifecycle substitution
- Denominator drift
- Runtime v2 or v0.93 work
