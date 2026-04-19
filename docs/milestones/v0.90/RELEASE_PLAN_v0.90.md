# Release Plan - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-16
- Owner: Daniel Austin
- Status: pre-third-party-review readiness

## Purpose

Record the intended release flow for v0.90 after `v0.89.1` WP-19 promoted the
planning package into tracked milestone docs and v0.90 WP-01 opened the issue wave.

## Release Flow

1. The `v0.89.1` WP-19 promotion gate promotes this planning package into tracked milestone docs.
2. WP-01 opens the official issue wave from this promoted package. This is now `#2019`, with WP-02 through WP-20 opened as `#2021` through `#2039`.
3. Sprint 1 lands supervisor, cycle, continuity, and operator-safety surfaces.
4. Sprint 2 lands inspection/status and the primary stock-league demo proof
   surfaces.
5. Sprint 3 lands bounded sidecar work: demo extensions, the `93%` coverage
   tranche, milestone compression pilot, and repo visibility prototype.
6. Sprint 4 completes docs, explicit Rust refactoring, internal review,
   pre-third-party readiness, third-party review, remediation, next planning,
   and ceremony.

## Quality Expectations

- Every code-bearing WP runs the smallest truthful Rust validation set.
- Every docs-only WP runs path/leakage and consistency scans.
- Demo WPs must name primary proof surfaces before claiming success.
- The coverage gate should ratchet to the intended rounded `93%` tranche only
  after measurement and proof are green. The current tracker reports `92.40%`,
  rounded to `93%`, and WP-10 validation also recorded `92.46%`.
- Milestone compression should prove state and drift checks before any broader
  write automation.
- Repo visibility should produce a bounded manifest and linkage report, not a
  full repo-indexing claim.
- Rust refactoring must be explicit, bounded, and validated.
- Release notes must not claim full identity, live trading, or general
  autonomous operation.

## Release Blockers

- The v0.90 WP-01 issue-wave update has landed.
- No demo extension should displace the stock-league proof as the primary
  long-lived runtime demo.
- No milestone-compression automation should silently mutate release truth.
- WP-10 coverage ratchet evidence is complete for pre-third-party review.
- WP-18 must complete the readiness handoff before WP-16 third-party review
  begins.
- No release ceremony should begin until next-milestone planning is complete.

## Exit Criteria

- issue wave complete or explicitly deferred
- demo matrix validated
- quality gate complete, including the rounded `93%` coverage tranche
- milestone compression and repo visibility pilots dispositioned
- Rust refactor validation complete
- review findings resolved or deferred
- release notes and tag ready
