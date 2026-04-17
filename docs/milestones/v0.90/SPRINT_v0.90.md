# Sprint Plan - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: issue wave open

## Sprint Goal

Deliver the first bounded long-lived-agent runtime package without overclaiming
identity, trace, query, or financial capability.

## Sprint Structure

### Sprint 1: Core Runtime Shape

Goal: make the long-lived runtime substrate explicit enough to implement.

Planned WPs:

- WP-01: milestone planning and issue wave
- WP-02: supervisor and heartbeat
- WP-03: cycle contract and artifact root
- WP-04: state and continuity handles
- WP-05: operator control and safety

Execution gate:

- WP-02 through WP-05 must inherit the runtime artifact, focused-test, and
  non-goal requirements in WP_EXECUTION_READINESS_v0.90.md before
  implementation.

### Sprint 2: Inspection And Demo Proof

Goal: produce a reviewer-legible long-lived proof surface.

Planned WPs:

- WP-06: minimal inspection and trace boundary
- WP-07: stock league demo scaffold
- WP-08: long-lived demo integration

Execution gate:

- WP-06 through WP-08 must produce reviewer-visible proof commands or proof
  packets, not only prose. The minimum proof surfaces are listed in
  WP_EXECUTION_READINESS_v0.90.md.

### Sprint 3: Sidecar Proof And Process Improvements

Goal: strengthen the milestone without disrupting the long-lived runtime core.

Planned WPs:

- WP-09: demo extensions and proof expansion
- WP-10: coverage ratchet to 93 percent
- WP-11: milestone compression pilot
- WP-12: repo visibility prototype

Execution gate:

- WP-09 must name selected demos or explicitly defer before execution. WP-10
  must measure first and ratchet second. WP-11 and WP-12 must remain bounded
  sidecars, as described in WP_EXECUTION_READINESS_v0.90.md.

### Sprint 4: Docs, Refactor, Review, And Release Tail

Goal: converge, review, remediate, plan the next milestone, and release.

Planned WPs:

- WP-13: docs and review pass
- WP-14: Rust refactoring pass
- WP-15: internal review
- WP-16: third-party review
- WP-17: findings remediation
- WP-18: final quality and release readiness
- WP-19: next milestone planning
- WP-20: release ceremony

## Blockers For The Current Promotion Gate

- Decide whether reasoning graph, signed trace, and TQL are core v0.90 scope or
  follow-on scope.
- Decide whether urgency/prioritization is a narrow scheduler feature or a later
  governance feature.
- Decide which new or extended demos belong in WP-09 without diluting the stock
  league proof path.
- Confirm the WP-10 coverage ratchet target remains `93%` for this revision.
- Confirm the WP-11 milestone-compression pilot stays read-only or low-mutation
  until it proves accurate against real issue and PR truth.
- Confirm the WP-12 repo visibility prototype is bounded to a manifest and one
  linkage proof rather than a full repo-indexing platform.
- Identify the Rust refactoring targets for WP-14 from real maintainability,
  testability, or review evidence.
- Keep WP_EXECUTION_READINESS_v0.90.md current as the tracked execution contract
  for the open WP wave.
- Convert this tracked package into tracked docs under docs/milestones/v0.90.
- Issue numbers are open and WP_ISSUE_WAVE_v0.90.yaml now records `#2019` and `#2021` through `#2039`.

## Demo / Review Plan

Primary demo:

- bounded long-lived stock league using fixture-backed or delayed/public data

Demo extension lane:

- WP-09 may add or extend selected demos as child issues, but each demo must
  have a named proof claim, validation command, and explicit non-goals.

Required review checks:

- no live trading
- no financial advice
- operator controls remain authoritative
- cycle artifacts are inspectable
- continuity claims remain pre-v0.92
- demo extensions do not overclaim beyond their proof artifacts
- milestone compression does not silently mutate release truth
- repo visibility distinguishes canonical docs from planning and historical
  residue

## Exit Criteria

- every sprint item maps to a WBS row
- every WBS row can become a bounded issue
- demo and proof surfaces are named before implementation begins
