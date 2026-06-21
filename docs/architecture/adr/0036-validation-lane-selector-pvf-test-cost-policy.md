# ADR 0036 Candidate: Validation Lane Selector And PVF Test-Cost Policy

- Status: Candidate
- Target milestone: v0.91.6
- Related issues: #4212, #4223, #4251
- Related ADRs: ADR 0024, ADR 0028, Candidate ADR 0032, Candidate ADR 0033
- Source evidence:
  - `docs/architecture/VALIDATION_LANE_SELECTOR.md`
  - `docs/milestones/v0.91.6/review/V0916_VALIDATION_MANAGER_TEST_TAX_MINI_SPRINT_REVIEW_4212.md`
  - `docs/milestones/v0.91.6/review/PVF_LONG_VALIDATION_LANE_INDEX_4223.md`
  - `docs/milestones/v0.91.6/review/V0916_COMPLETED_SPRINT_RETAINED_EVIDENCE_MATRIX_4251.md`

## Context

v0.91.6 exposed validation cost as a project blocker. The test surface expanded
quickly, especially around modular Rust binaries and lifecycle tooling. Running
the whole suite for every change is too slow, but skipping proof would damage
C-SDLC trust.

The intended architecture is not "less validation." It is better validation
selection: every issue and PR should run the smallest deterministic proof lane
that covers the touched surface, while ambiguous or release-critical surfaces
escalate instead of returning false confidence.

## Decision

ADL should treat validation lane selection as an architecture boundary.

Normal issue work should use a tracked validation profile selected from
declared surface metadata, issue scope, changed paths, proof role, and release
gate status. The selected lane must be deterministic, reviewable, and recorded
in the issue evidence.

The selector may choose focused docs, prompt-card, owner-binary, runtime,
provider, review, or retained-evidence checks when those lanes prove the
changed surface.

The selector must fail closed or escalate when:

- touched paths cross shared Rust/runtime/tooling boundaries
- release-gate proof is involved
- provider credentials, live external systems, or long-running lanes are needed
- lane metadata is missing or contradictory
- generated evidence could hide skipped, pending, blocked, or failed proof

## Consequences

### Positive

- Makes large validation surfaces operationally survivable.
- Preserves proof while avoiding reflexive full-suite runs for narrow changes.
- Gives future validation-manager agents an explicit policy target.
- Connects PVF lane truth to concrete PR validation decisions.

### Negative

- Test authors must classify new tests and lanes as part of authoring.
- Incorrect metadata can become a validation risk.
- Shared code changes will still need broader proof and cannot be forced into
  a narrow lane for speed.

## Alternatives Considered

### Always run the full suite

This is simple but blocks the development model as test count grows.

### Let developers choose ad hoc commands

This is faster in the moment, but loses deterministic proof selection and makes
review evidence hard to trust.

### Use path-string matching alone

This is rejected as brittle. Path evidence can contribute to selection, but the
policy must be backed by declared lane metadata and proof roles.

## Validation Notes

Promotion should review the selector design, mini-sprint review, long-lane
index, and retained-evidence matrix. It should confirm that focused validation
does not claim release readiness when release-gate or ambiguous surfaces are
unproven.

## Non-Claims

- This ADR does not implement the future validation manager agent.
- This ADR does not reduce CI branch-protection requirements by itself.
- This ADR does not allow pending, skipped, deferred, or blocked proof to count
  as passed.
- This ADR does not claim every existing test is fully classified.
