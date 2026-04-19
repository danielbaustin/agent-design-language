# Release Notes - v0.90 Draft

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: pre-third-party-review readiness

## Draft Summary

v0.90 is the first bounded long-lived-agent runtime milestone.

The release story being assembled is:

- supervised agents can run across bounded cycles
- each cycle emits reviewable artifacts
- continuity is explicit without claiming full identity
- operators can inspect and stop long-lived execution
- the stock league demo proves recurring supervised behavior safely
- selected demo extensions strengthen proof coverage without displacing the
  stock-league proof
- coverage ratchets to the intended `93%` tranche by rounded workspace evidence
  while preserving the existing workspace and per-file gates
- milestone compression and repo visibility land as bounded pilots
- Rust refactoring is explicit, scoped, and validated

## Landed Highlights So Far

- long-lived supervisor and heartbeat
- cycle manifest and artifact contract
- state and continuity handle package
- operator control and guardrail surfaces
- stock league demo proof package
- demo extension proof package
- milestone compression pilot
- repo visibility manifest and linkage report
- coverage ratchet evidence: `92.40%` current tracker value, rounded to `93%`,
  with gates passing and no active file-floor exclusion
- Rust refactoring wave: one `RATIONALE`, nineteen `REVIEW`, and fourteen
  `WATCH` tracker items after the WP-14 child splits
- internal review and accepted internal-finding remediation

Still open before final release copy:

- third-party review
- third-party findings remediation, if findings are accepted
- final next-milestone planning update
- release ceremony

## Explicit Non-Claims

v0.90 should not claim:

- full v0.92 identity/capability substrate
- live trading
- financial advice
- unbounded autonomy
- full autonomous release approval or silent closeout automation
- full repo semantic indexing
- full signed-trace or TQL completion unless those are explicitly promoted and
  implemented in the issue wave

## Release Status

Draft. Runtime, demo, sidecar, coverage, refactor, docs, and internal-review
proof work has landed. WP-18 is preparing the third-party review handoff; the
third-party review, any accepted findings remediation, next-milestone planning
finalization, and release ceremony remain open, so this file must not yet be
treated as final release copy.
