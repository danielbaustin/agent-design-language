# ADR 0032: Parallel Validation Fabric

- Status: Accepted
- Date: 2026-06-23
- Accepted in: v0.91.6
- Candidate source: docs/architecture/adr/0032-parallel-validation-fabric.md
- Target milestone: v0.91.4
- Related issues: #3398, #3399, #3400, #3401, #3402, #3403, #3404, #3406, #3437, #3444
- Related ADRs: ADR 0024, ADR 0028, ADR 0029
- Source evidence:
  - `docs/milestones/v0.91.4/features/PARALLEL_VALIDATION_FABRIC.md`
  - `docs/milestones/v0.91.4/features/PVF_VALIDATION_LANE_TAXONOMY_v0.91.4.md`
  - `docs/milestones/v0.91.4/features/PVF_VALIDATION_LANE_MANIFEST_SCHEMA_v0.91.4.json`
  - `docs/milestones/v0.91.4/FIVE_MINUTE_SPRINT_REPEATABILITY_REPORT_2026-05-27.md`

## Context

The v0.91.3 and v0.91.4 workflow improvements make short sprint loops
plausible, but validation can still dominate wall-clock time. A five-minute
sprint is not operationally convincing if every transition waits for one
monolithic long-running proof gate.

The solution cannot be "skip tests." ADL needs faster validation while making
proof more explicit, not less explicit.

## Decision

ADL should adopt Parallel Validation Fabric as the architecture boundary for
decomposing validation into lane-scoped proof.

Each validation lane should declare:

- owner or responsible issue
- input files, fixtures, or evidence roots
- cache key or invalidation rule when applicable
- synchronization barrier, if any
- expected proof artifact
- status vocabulary for passed, failed, pending, deferred, and blocked proof

Aggregate status must be derived from lane truth. A green aggregate cannot hide
a failing, pending, or blocked lane.

## Consequences

### Positive

- Makes validation parallelizable without making proof vague.
- Gives sprint and issue records a shared vocabulary for pending, deferred,
  blocked, failed, and passed proof.
- Supports cache-aware evidence reuse only when identity and policy prove
  equivalence.
- Creates a path toward shorter C-SDLC wall-clock cycles without weakening
  branch protection or human review.

### Negative

- Validation manifests and lane records add another proof surface to maintain.
- Cache-aware proof can be dangerous if identity, freshness, tooling, or policy
  inputs are incomplete.
- CI fallback behavior must be fail-closed.

## Alternatives Considered

### Keep validation monolithic

This is simple, but it preserves the validation-tail bottleneck.

### Skip duplicate validation when local tests passed

This is unsafe unless exact commit/tree identity, policy inputs, tool versions,
commands, logs, and freshness prove equivalence. Without that proof, remote CI
must run.

## Validation Notes

This candidate should be reviewed against the PVF lane taxonomy, lane manifest
schema, example manifest, five-minute sprint repeatability report, and WP-14
quality-gate evidence.

## Non-Claims

- This ADR does not claim distributed validation scheduling is complete.
- This ADR does not replace CI, branch protection, human review, or closeout.
- This ADR does not allow pending or deferred proof to be counted as passed.
