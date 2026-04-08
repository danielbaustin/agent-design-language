# [v0.87.1][WP-02] Runtime environment completion

Task ID: issue-1436
Run ID: issue-1436
Version: v0.87.1
Title: [v0.87.1][WP-02] Runtime environment completion
Branch: codex/1436-v0-87-1-wp-02-runtime-environment-completion
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T17:46:00Z
- End Time: 2026-04-08T18:05:16Z

## Summary
Added one authoritative local runtime-environment contract in Rust and aligned the `v0.87.1` docs to it. The runtime now has an explicit `RuntimeEnvironment` surface with default roots, env overrides, and a sanitized runtime marker, and the existing run-artifact path logic now resolves through that surface instead of leaving runtime-root behavior implicit.

## Artifacts produced
- `adl/src/runtime_environment.rs`
- `adl/src/artifacts.rs`
- `adl/src/lib.rs`
- `docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT.md`
- `docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`
- `docs/milestones/v0.87.1/README.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`

## Actions taken
- added `adl::runtime_environment::RuntimeEnvironment` as the authoritative runtime-root and run-artifact-root contract
- defined default roots `.adl/` and `.adl/runs/` plus bounded overrides via `ADL_RUNTIME_ROOT` and `ADL_RUNS_ROOT`
- added runtime marker publication at `.adl/runtime_environment.json` with sanitized path labels that avoid leaking absolute host paths
- routed `artifacts::runs_root()` through the new runtime-environment surface
- ensured default run-layout creation also materializes the runtime-environment marker
- aligned the runtime-environment feature docs and milestone docs to the concrete code contract rather than broader aspirational continuity claims
- removed a flaky duplicate CLI-side marker assertion and kept the behavior covered by focused `runtime_environment` unit tests

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet
- Worktree-only paths remaining:
  - `adl/src/runtime_environment.rs`
  - `adl/src/artifacts.rs`
  - `adl/src/lib.rs`
  - `docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT.md`
  - `docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`
  - `docs/milestones/v0.87.1/README.md`
  - `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: bounded worktree update pending `pr finish`
- Verification performed:
  - `git status --short`
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` to confirm the new runtime-environment module and doc-touching code stayed formatted
  - `cargo test --manifest-path adl/Cargo.toml runtime_environment_defaults_to_repo_local_roots -- --nocapture` to verify the default runtime roots remain repo-local and deterministic
  - `cargo test --manifest-path adl/Cargo.toml runtime_environment_respects_env_overrides_without_leaking_absolute_paths_in_marker -- --nocapture` to verify env overrides are supported without leaking absolute host paths into the marker
  - `cargo test --manifest-path adl/Cargo.toml run_artifacts_root_points_to_repo_adl_runs -- --nocapture` to verify the existing run-artifact root behavior still points at `.adl/runs`
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` to verify the new runtime-environment surface and tests are warning-free
- Results:
  - formatting check passed
  - both runtime-environment unit tests passed
  - the run-artifact-root regression passed
  - clippy passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo test --manifest-path adl/Cargo.toml runtime_environment_defaults_to_repo_local_roots -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml runtime_environment_respects_env_overrides_without_leaking_absolute_paths_in_marker -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml run_artifacts_root_points_to_repo_adl_runs -- --nocapture"
      - "cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed:
  - `cargo test --manifest-path adl/Cargo.toml runtime_environment_defaults_to_repo_local_roots -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml runtime_environment_respects_env_overrides_without_leaking_absolute_paths_in_marker -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml run_artifacts_root_points_to_repo_adl_runs -- --nocapture`
- Fixtures or scripts used:
  - the focused `runtime_environment` unit tests in `adl/src/runtime_environment.rs`
  - the run-state regression in `adl/src/cli/tests/run_state/basics.rs`
- Replay verification (same inputs -> same artifacts/order):
  - the runtime-environment tests recheck the same root-resolution rules and sanitized marker output for fixed inputs
- Ordering guarantees (sorting / tie-break rules used):
  - root selection follows a fixed precedence of `ADL_RUNTIME_ROOT` / `ADL_RUNS_ROOT` overrides before repo-local defaults
- Artifact stability notes:
  - the runtime marker writes repo-relative labels for repo-local roots and stable external placeholders for override mode, so marker shape stays stable without host-path leakage

## Security / Privacy Checks
- Secret leakage scan performed:
  - reviewed the new runtime marker fields and related docs/tests to ensure they do not include secrets or tokens
- Prompt / tool argument redaction verified:
  - the runtime marker and docs do not record prompts or tool arguments
- Absolute path leakage check:
  - the override-mode marker intentionally records `external_runtime_root` / `external_runs_root` instead of absolute host paths
- Sandbox / policy invariants preserved:
  - yes; the change stayed within bounded runtime-root, artifact-path, test, and doc surfaces

## Replay Artifacts
- Trace bundle path(s):
  - not applicable; this issue defines runtime roots rather than trace replay content
- Run artifact root:
  - `.adl/runs/` by default, or the configured `ADL_RUNS_ROOT` override
- Replay command used for verification:
  - the focused runtime-environment and run-state regression commands listed above
- Replay result:
  - passed; the same fixed inputs preserved root-selection and marker-shape behavior

## Artifact Verification
- Primary proof surface:
  - `adl::runtime_environment::RuntimeEnvironment` and its runtime marker at `.adl/runtime_environment.json`
- Required artifacts present:
  - yes; the runtime-environment code surface and aligned docs are present in the worktree
- Artifact schema/version checks:
  - the new runtime marker schema is explicit as `runtime_environment.v1`
- Hash/byte-stability checks:
  - not separately run; the focused proof standard here was stable root resolution and sanitized marker shape
- Missing/optional artifacts and rationale:
  - no runnable D1 demo script landed in this issue because WP-02 establishes the substrate contract first; demo implementation remains a later milestone proof step

## Decisions / Deviations
- kept the runtime contract bounded to local runtime roots, run-artifact roots, and marker publication instead of widening into lifecycle or Shepherd policy
- tightened the docs so `v0.87.1` does not overclaim full chronosense or persistent identity in the runtime-environment layer
- removed a flaky duplicate CLI-side marker test once the behavior was covered more cleanly by focused `runtime_environment` unit tests

## Follow-ups / Deferred work
- later Sprint 1 issues should build lifecycle, trace, resilience, and review behavior on top of this runtime-environment contract rather than adding new root-selection logic
