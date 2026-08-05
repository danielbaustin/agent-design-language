# Issue 5812 Freedom Gate Clippy Repair Design

## Change Boundary

The only product edit is in `adl/src/csm_freedom_gate.rs`: replace the two
`unwrap_or_else` calls at the retained-status projection boundary with eager
`unwrap_or` values. The JSON defaults remain exactly `true` for
`executor_requires_gate_decision` and `false` for
`unmediated_execution_allowed`.

The nearby module tests already assert those fail-closed defaults and unsafe
retained-artifact rejection. They are the behavioral proof surface. The named
`adl-gws-context-mirror` Clippy target is the warning reproduction surface.

## Execution Steps

1. Confirm the two expressions and their tests still match the issue evidence.
2. Apply only the two eager-default substitutions.
3. Run the module tests, formatting check, and production-binary Clippy with
   warnings denied.
4. Inspect the exact diff for lockfile, dependency, formatting, or unrelated
   source churn.

## Invariants And Stop Conditions

- Runtime output and fail-closed behavior do not change.
- `Cargo.toml`, `Cargo.lock`, dependencies, Google Drive code, and adjacent
  Freedom Gate semantics are outside scope.
- Stop if current source no longer contains the two reported warnings or if a
  focused test exposes any semantic change.

## Rollback And Proof

Rollback is the two-line revert. Completion requires focused unit tests that
assert both defaults and unsafe-artifact rejection, `cargo fmt --check`, the
exact Clippy command with `-D warnings`, diff hygiene, and bounded review.
