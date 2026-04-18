# Release Notes - v0.90 Draft

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: issue wave open

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
- coverage ratchets to `93%` if measurement and validation support it
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

Still open before final release copy:

- `93%` coverage tranche
- explicit Rust refactoring rollup
- internal review, third-party review, findings remediation, final readiness,
  next-milestone planning, and release ceremony
- Rust refactoring validation record

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

Draft. Runtime and demo proof work has landed through WP-09, and WP-11/WP-12
sidecar proofs have landed. WP-10 and the review/release tail remain open, so
this file must not yet be treated as final release copy.
