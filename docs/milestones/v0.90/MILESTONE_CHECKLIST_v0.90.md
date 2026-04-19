# Milestone Checklist - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: release-tail pre-ceremony

## Planning Gate

- [x] The `v0.89.1` WP-19 promotion gate promotes this package into tracked milestone docs.
- [x] FEATURE_DOCS_v0.90.md has current promoted/deferred dispositions.
- [x] WP_ISSUE_WAVE_v0.90.yaml has real issue titles, dependencies, and issue numbers.
- [x] Every feature doc has a WBS row or a deferred/later-band home.
- [x] Every idea doc has a reader-facing or deferred home.
- [x] WP_EXECUTION_READINESS_v0.90.md binds open WPs to concrete source docs,
  artifact/proof surfaces, validation expectations, and non-goals.

## Scope Gate

- [x] v0.90 thesis is confirmed.
- [x] Full v0.92 identity is explicitly out of scope.
- [x] Live trading and financial advice are explicitly out of scope.
- [x] Trace/TQL/reasoning graph scope is accepted, narrowed, or deferred.
- [x] Demo extension scope is named and bounded.
- [x] Coverage ratchet target is confirmed as the rounded `93%` tranche for this revision.
- [x] Milestone compression scope is limited to canonical state and drift checks unless explicitly widened.
- [x] Repo visibility scope is limited to a bounded manifest/linkage prototype.
- [x] Rust refactoring targets are justified by maintainability, testability, or review evidence.
- [x] WP cards have a tracked execution-readiness gate that prevents generic
  runtime, demo, coverage, compression, visibility, and refactor work.

## Implementation Gate

- [x] Supervisor/heartbeat work lands.
- [x] Cycle contract work lands.
- [x] Continuity handle work lands.
- [x] Operator control and safety work lands.
- [x] Demo scaffold lands.
- [x] Demo integration lands.
- [x] Demo extension lane lands or is explicitly deferred.
- [x] Coverage ratchet reaches the rounded `93%` tranche only after measurement and proof are green.
- [x] Milestone compression pilot lands without autonomous release approval.
- [x] Repo visibility prototype lands with a reviewer-readable manifest and linkage report.
- [x] Rust refactoring pass lands with validation evidence.

## Review And Release Gate

- [x] Quality gate is green through third-party review.
- [x] Docs review is complete through third-party review.
- [x] Rust refactor validation is complete.
- [x] Milestone compression drift checks are clean or explicitly dispositioned.
- [x] Repo visibility linkage report is reviewed.
- [x] Internal review is complete.
- [x] Third-party review is complete.
- [x] Findings remediation is complete.
- [ ] Final quality and release readiness pass is complete for release ceremony.
- [x] Next milestone planning is complete before release ceremony.
- [ ] Release ceremony is complete.

## Current Tracker Snapshot

- Coverage tracker: workspace line coverage is `92.40%`, rounded to the
  intended `93%` tranche; workspace and per-file gates pass with no active
  file-floor exclusion. WP-10 validation also recorded `92.46%`.
- Rust tracker: one `RATIONALE`, nineteen `REVIEW`, and fourteen `WATCH` items
  remain after the WP-14 child refactoring wave.
- Closeout: an earlier closeout pass reconciled almost all closed issue cards;
  remaining open issue work is normal release ceremony work.
