# Structured Task Prompt

Template: 1.0.0

Issue: 5411

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the four #5227 review findings retained by #5411 and their direct proving surfaces.

## Deliverables

- Truthful selector and release-evidence records
- Process-group guardian with bounded shutdown and capture
- Periodic resource-pressure monitor wired to signed continuity and graceful stop
- Focused regression and integration proof

## Acceptance

1. Selector claims accurately distinguish reporting from invocation without modifying Runtime v2
2. Guardian terminates the supervised process tree and bounds output-capture shutdown
3. StopRequired pressure creates a signed live checkpoint before graceful kernel shutdown
4. Checkpoint failure cannot be reported as a clean pressure stop
5. Executed, contract-only, ignored, and deferred evidence are distinct and only executed proof closes live requirements
6. Runtime v3 remains within the approved 12,000-line implementation budget or any variance is explicitly justified

## Dependencies

- #5410 merged and closed
- #5409 protected paths remain untouched
- Existing Runtime v3 weather, continuity, supervisor, and guardian contracts

## Inputs

- docs/reviews/v0.91.7/runtime-v3-5410/DESIGN.md
- docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md
- docs/architecture/runtime_v3_release_proof_gate_5220.v1.json
- adl-runtime/src/guardian.rs
- adl-runtime-kernel/src/weather.rs
- adl-runtime-kernel/src/live_continuity.rs

## Non Goals

- Runtime v2 changes or decommission
- Default runtime cutover
- New service-manager, signing, serialization, or monitoring framework
- GPU qualification or distributed-polis work
