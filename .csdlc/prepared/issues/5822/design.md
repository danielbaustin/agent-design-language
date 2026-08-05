# WP-05 C-SDLC Estimation And Cycle-Time Reduction Design

## Outcome And Existing Evidence

Issue #5822 reconnects previously delivered session telemetry and deterministic
prediction work to the independent C-SDLC v2 lifecycle. Historical scripts
under `adl/tools/skills/sprint-conductor/scripts/` are evidence and migration
inputs only; they are not restored as lifecycle authority. Current static
planning estimates live in `csdlc-v2/src/cards.rs` and remain fallback defaults
until a measured candidate is proven better.

The outcome is an advisory, provenance-bearing estimate path plus a measured
operator workflow simplification. Estimates never stop work, advance lifecycle
state, mark a goal complete, or override validation, review, merge, or closeout.

## Typed Estimation Boundary

The v2 implementation introduces typed observation, forecast, and outcome
schemas that join available data from:

- C-SDLC lifecycle phase and card records;
- Git/GitHub publication, check, review, merge, and closure timing;
- PVF and validation lane duration/outcome records;
- approved session telemetry with model/platform-era labels;
- explicit operator annotations for pauses, blockers, and scope changes.

Every field retains source provenance and unknown values remain unknown. The
forecast reports point estimates, ranges, cohort size, dispersion, confidence,
outlier factors, and drift state. Comparable-issue selection is deterministic
and excludes the target issue's own future actuals.

## Execution Design

1. Inventory historical predictor behavior and build a joined fixture corpus
   with unavailable, interrupted, multi-session, and schema-drift cases.
2. Add v2-owned typed schemas and deterministic collection/adaptation logic.
3. Implement comparable-cohort selection and a robust sparse-data baseline;
   preserve static planning profiles as an explicit fallback.
4. Add a typed SPP reference/disposition for an accepted advisory forecast.
5. Record forecast-versus-actual outcomes at truthful terminal closeout without
   turning estimation into enforcement.
6. Measure baseline and candidate operator cycle time over equivalent workflow
   cohorts, including active work, validation, review, CI, and wait components.

The implementation owns the exact integration-test target
`csdlc-v2/tests/estimation_contracts.rs`. That target must cover schema and
round-trip behavior, privacy and leakage negatives, deterministic cohort
selection, sparse-data fallback, calibration, and backtesting. Validation uses
nextest with `--no-tests=fail`; substring filters are not acceptable proof.

## Invariants And Stop Conditions

- No transcript body or sensitive path is copied into tracked evidence.
- Missing data, schema drift, and model-era changes are explicit.
- No sunset wrapper becomes operational authority.
- No speedup claim is made without comparable baseline and candidate cohorts.
- Stop if joined provenance is ambiguous, the estimator leaks target actuals,
  or workflow simplification weakens a lifecycle gate.

## Rollback And Proof

The candidate estimator and simplified path remain feature-bounded until
backtests and workflow comparisons pass. Rollback restores static profiles and
the existing v2 route without data loss. Proof covers deterministic schemas,
unknown/schema-drift negatives, cohort leakage rejection, forecast stability,
non-enforcement, typed card round trips, closeout comparison, and measured
cycle-time improvement.
