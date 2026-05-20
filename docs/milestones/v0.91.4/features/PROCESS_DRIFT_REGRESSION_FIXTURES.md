# Process Drift Regression Fixtures

## Status

Planned `v0.91.4` feature.

## Purpose

Prevent the C-SDLC process from drifting back into ambiguous card semantics,
stale outcome records, skipped closeout, or local-only proof.

The process should fail closed when its core truth surfaces are wrong.

## Scope

`v0.91.4` should include fixtures for:

- legacy `SRP` policy wording in new bundles
- stale `SOR` integration state
- missing review results
- skipped child closeout
- sprint umbrella state that overclaims cleanliness
- overlapping shard write surfaces
- local-only durable evidence
- missing or unverifiable signed trace proof

## Acceptance Criteria

- Regression fixtures run through a focused validation command.
- Known drift modes are blocked, routed, or reported with clear diagnostics.
- Fixture expectations are tracked and reviewer-readable.
- The quality gate requires these fixtures before default operation is claimed.
- The release packet records fixture results.

## Non-Goals

- This feature does not require broad full-repo tests for every docs-only
  change.
- This feature does not replace human review.
- This feature does not treat fixture coverage as proof that every future
  process failure is impossible.
