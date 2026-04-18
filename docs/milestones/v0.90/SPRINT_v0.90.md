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

## Current Promotion Gate Status

- Reasoning graph, signed trace, TQL, and urgency/prioritization are accounted
  for as deferred or later-band context unless promoted by a later issue.
- WP-09 selected a bounded stock-league reviewer evidence extension without
  diluting the primary stock-league proof path.
- WP-10 remains open and must still prove the `93%` coverage tranche before any
  release-readiness claim.
- WP-11 landed a read-only milestone-compression pilot.
- WP-12 landed a bounded manifest and one linkage proof rather than a full
  repo-indexing platform.
- WP-14 refactoring targets are drawn from the maintainability hotspot plan and
  active child issues.
- WP_EXECUTION_READINESS_v0.90.md remains the tracked execution contract for the
  open WP wave.
- Issue numbers are open and WP_ISSUE_WAVE_v0.90.yaml records `#2019` and
  `#2021` through `#2039`.

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
- demo and proof surfaces are named and their status is kept aligned with
  landed or deferred work
