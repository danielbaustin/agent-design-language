# Runtime v3 API-auth coverage mapping

## Problem

`adl-runtime/src/runtime_api_auth.rs` has focused tests in the independent
Runtime v3 crate, but the PR-fast coverage impact map does not know that
ownership boundary. Mixing the mapping repair with Runtime v3 source changes
escalates the PR to full legacy-workspace coverage and obscures the focused
proof.

## Design

- Add one `runtime_v3_auth` risk selector for `runtime_api_auth.rs`.
- Resolve that selector to `test(/^runtime_api_auth::tests::/)`.
- Run an auth-only expression against `adl-runtime/Cargo.toml`.
- Preserve the existing ADL workspace run when any non-auth selector is also
  present.
- Cover both auth-only and mixed selection in shell contract tests.

## Boundaries

- No runtime source changes.
- No weather-service changes.
- No legacy full-coverage flake repair.
- No AWS execution.

