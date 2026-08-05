# Issue 5788 design

Status: approved for bounded execution.

Keep repository-native Cargo invocations lock-preserving by default. The owner
binary installer uses only current `adl/Cargo.toml` binary targets, always
passes `--locked`, snapshots the caller's exact pre-invocation lockfile bytes,
and restores only drift created during that invocation. Existing user-owned
lockfile changes remain untouched. The owner validation lane shares the same
current inventory and lock-preserving build semantics.

Focused shell fixtures replace Cargo with deterministic fakes so regressions
prove removed-target failure and dependency-resolution churn without rebuilding
the repository binaries.
