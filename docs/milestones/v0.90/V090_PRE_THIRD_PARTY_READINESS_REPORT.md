# v0.90 Pre-Third-Party Readiness Report

## Metadata

- Milestone: v0.90
- Work package: WP-18
- Issue: #2037
- Status: pre-third-party-review readiness
- Date: 2026-04-18
- Scope: quality, demo, docs, compression, visibility, coverage, Rust refactor, closeout, and release-truth checks before WP-16 third-party review

## Readiness Summary

v0.90 is ready to hand to third-party review after this WP-18 branch merges.
The milestone is not released yet. The correct remaining sequence is:

- WP-16 third-party review
- WP-17 findings remediation if findings are accepted
- WP-19 next-milestone planning finalization
- WP-20 release ceremony

The implementation, demo, sidecar, coverage, Rust-refactor, docs, and internal
review surfaces are no longer open-ended. The live issue list shows only the
normal review/release-tail issues remaining: #2035, #2036, #2037, #2038, and
#2039.

## Current Tracker Numbers

Coverage tracker:

- Current workspace line coverage: `92.40%`, rounded to the intended `93%`
  tranche.
- Workspace gate: pass.
- Per-file gate: pass.
- Active file-floor exclusions: none.
- WP-10 validation also recorded a workspace line-coverage run at `92.46%`.
- Focused coverage child work closed for #2121, #2122, and #2123.

Test tracker evidence:

- #2121 added focused coverage for `adl/src/godel/stage_loop.rs`; the recorded
  focused validation passed with 10 tests and the adjacent stage-loop execution
  proof passed.
- #2122 added focused coverage for `adl/src/cli/godel_cmd.rs`; the recorded
  focused and replay validation passed with 9 command-path tests after the new
  single missing-argument test passed.
- #2123 added focused coverage for `adl/src/cli/pr_cmd.rs`; the recorded
  focused validation passed with 123 tests and zero failures in the filtered
  run.

Rust tracker:

- Current status after the WP-14 child split wave: one `RATIONALE` item,
  nineteen `REVIEW` items, and fourteen `WATCH` items.
- This is a material improvement from the previous scan, where four files were
  in `RATIONALE`.
- The latest split wave reduced `long_lived_agent.rs`,
  `identity_cmd/tests.rs`, `demo/stock_league.rs`,
  `internal_commands.rs`, and `tooling_cmd/tests.rs` through child issues.

Closeout tracker:

- The earlier closeout pass reconciled almost all closed issue cards.
- Remaining open issues are normal release-tail work, not stale completed cards.
- WP-14 is intentionally closed as an umbrella issue because its implementation
  landed through child PRs.

## Gate Review

Quality gate:

- Latest daily preflight passed on 2026-04-18 with shell syntax checks, Rust
  format, Rust clippy with warnings denied, and the Rust test suite.
- Coverage evidence is current enough for pre-third-party review and is
  explicitly recorded as a rounded `93%` tranche rather than an overprecise
  threshold claim.

Demo gate:

- D1 through D5 landed the long-lived supervisor, cycle contract, operator
  controls, stock-league integration, and proof expansion rows.
- D6 landed the repo-visibility proof packet.
- Demo claims remain bounded: no live trading, no financial advice, no full
  v0.92 identity claim, and no unbounded autonomy.

Docs gate:

- The root README, runtime README, changelog, v0.90 README, sprint plan, WBS,
  issue wave, checklist, release plan, release notes, compression state, and
  repo-visibility manifest now point at the same release truth.
- WP-18 now explicitly precedes WP-16 in the review-tail dependency graph so
  the third-party reviewer receives the final pre-review packet rather than a
  stale post-review placeholder.

Compression gate:

- The milestone compression packet remains a read-only pilot.
- It records canonical milestone state and drift checks; it does not approve
  releases, merge PRs, close issues, or mutate release truth silently.

Repo visibility gate:

- The repo-visibility packet remains bounded to the v0.90 long-lived runtime
  slice.
- It links canonical milestone docs to implementation, tests, demos, and review
  surfaces without claiming full repository semantic indexing.

Rust refactor gate:

- WP-14 is complete through child issues and tracker evidence.
- Remaining large files are tracked as follow-up candidates, not v0.90 release
  blockers unless a later review finding promotes them.

Review gate:

- Internal review completed.
- Accepted internal findings were addressed before this readiness report.
- Third-party review has not started in the tracked issue wave until WP-18
  merges.

## Residual Risks

- Third-party review may still find new issues. Those belong to WP-17 if
  accepted.
- Coverage is a rounded `93%` tranche, not a literal `93.00%` or higher
  measurement. Release copy should preserve that nuance.
- The Rust tracker still has one `RATIONALE` file. That is a managed
  maintainability follow-up, not a release blocker after the WP-14 split wave.
- Final release readiness is not complete until WP-16, WP-17 if needed, WP-19,
  and WP-20 finish.

## Handoff Instruction

Use this report as the WP-16 third-party-review starting point. Reviewers
should focus on:

- whether the long-lived runtime claims match the landed implementation and
  demos
- whether coverage and Rust tracker claims are stated with the right precision
- whether no v0.90 doc overclaims full identity, live trading, financial advice,
  unbounded autonomy, or autonomous release approval
- whether release notes describe actual milestone scope rather than aspirational
  later-band work
