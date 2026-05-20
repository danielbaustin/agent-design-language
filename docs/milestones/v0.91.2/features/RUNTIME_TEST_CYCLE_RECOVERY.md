# Runtime And Test-Cycle Recovery

## Metadata

- Feature Name: Runtime/Test-Cycle Recovery And Coverage Ergonomics
- Milestone Target: `v0.91.2`
- Status: WP-04 and WP-05 landed
- Planned WP Home: WP-04, WP-05
- Source Docs: `.adl/docs/TBD/tools/RUNTIME_V2_TEST_CYCLE_RECOVERY_PLAN.md`; `.adl/docs/TBD/v0.90.5_TEST_RUNTIME_REDUCTION_PLAN.md`
- Proof Modes: CI evidence, tests, report
- Current proof surfaces: `docs/milestones/v0.91.2/review/runtime_test_cycle_recovery_report.md`; `docs/milestones/v0.91.2/review/coverage_gate_ergonomics_report.md`; `docs/milestones/v0.91.2/CI_RUNTIME_BUDGETS_v0.91.2.md`

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

## Current State

The WP-04 runtime/test-cycle recovery slice is now landed via:

- `#3042` / PR `#3048` shared-registry slow-test consolidation
- `#3043` / PR `#3049` proof-materialization slow-lane split
- `#3044` / PR `#3050` PR-fast selector tightening
- `docs/milestones/v0.91.2/review/runtime_test_cycle_recovery_report.md`

Supporting sibling evidence from `WP-05A`:

- `#3045` / PR `#3052` nextest timing diagnostics

The `WP-05` coverage ergonomics slice is now landed via:

- improved actionable next-step diagnostics in `adl/tools/check_coverage_impact.sh`
- focused regression coverage in `adl/tools/test_check_coverage_impact.sh`
- `docs/milestones/v0.91.2/review/coverage_gate_ergonomics_report.md`

Post-sprint follow-on `#3133` continues this line by tightening the ordinary
PR-fast nextest selector so structural module-barrel wiring, such as
`adl/src/lib.rs`, does not force a full nextest sweep when the real changed
runtime surface is still narrowly mapped.

Post-sprint follow-on `#3134` adds job/step-level CI runtime budget
observability so future slowdowns can be routed to lane selection, setup/cache,
Rust test execution, coverage execution, tooling contracts, or reporting
instead of being treated as anecdotal PR pain.
