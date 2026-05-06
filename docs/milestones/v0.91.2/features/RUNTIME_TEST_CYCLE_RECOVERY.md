# Runtime And Test-Cycle Recovery

## Metadata

- Feature Name: Runtime/Test-Cycle Recovery And Coverage Ergonomics
- Milestone Target: `v0.91.2`
- Status: planned
- Planned WP Home: WP-04 and WP-05
- Source Docs: `.adl/docs/TBD/tools/RUNTIME_V2_TEST_CYCLE_RECOVERY_PLAN.md`; `.adl/docs/TBD/v0.90.5_TEST_RUNTIME_REDUCTION_PLAN.md`
- Proof Modes: CI evidence, tests, report

## Purpose

Reduce redundant or overbroad proof phases without weakening release truth.
This feature responds directly to expensive coverage/test cycles that slowed
recent milestones.

## Scope

In scope:

- Analysis of redundant authoritative phases.
- Safe reductions or filters.
- Changed-source coverage diagnostics.
- Focused-test guidance for low-coverage modified files.

Out of scope:

- Silent threshold waivers.
- Weakening release evidence.
- Treating green docs-only checks as full coverage proof.

## Acceptance Criteria

- Runtime reductions preserve required proof coverage.
- Coverage failures point to actionable files and tests.
- Before/after runtime evidence is recorded.
