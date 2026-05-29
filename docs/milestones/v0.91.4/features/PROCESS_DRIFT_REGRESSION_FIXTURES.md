# Process Drift Regression Fixtures

## Status

Tracked WP-12 feature packet proposed for `v0.91.4`.

## Purpose

Prevent the C-SDLC process from drifting back into ambiguous card semantics,
stale outcome records, skipped closeout, or local-only proof.

The process should fail closed when its core truth surfaces are wrong.

## Scope

This WP-12 slice lands fixtures for:

- legacy `SRP` policy wording in new bundles
- stale `SOR` integration state
- missing review results
- skipped child closeout
- sprint umbrella state that overclaims cleanliness

## Acceptance Criteria

- Regression fixtures run through a focused validation command.
- Known drift modes are blocked, routed, or reported with clear diagnostics.
- Fixture expectations are tracked and reviewer-readable.
- The quality gate requires these fixtures before default operation is claimed.
- The release packet records fixture results.

## Focused Validation Command

Use the bounded regression command:

```bash
bash adl/tools/test_process_drift_regressions.sh
```

That command intentionally stays small and composes the existing focused
fixtures instead of inventing a broad new integration suite.

## Covered Drift Modes

- legacy `SRP` policy wording in new bundles
- stale `SRP` review truth
- stale `SOR` closeout truth
- skipped sprint child closeout and stale sprint-state truth
- machine-local absolute host-path leakage in durable prompt cards

## Proof Surfaces

- `adl/tools/test_process_drift_regressions.sh`
- `adl/tools/test_sprint_conductor_helpers.sh`
- `adl/src/cli/tooling_cmd/tests/structured_prompt.rs`
- `docs/milestones/v0.91.4/PROCESS_DRIFT_REGRESSION_REPORT_2026-05-28.md`

## Non-Goals

- This feature does not require broad full-repo tests for every docs-only
  change.
- This feature does not replace human review.
- This feature does not treat fixture coverage as proof that every future
  process failure is impossible.
