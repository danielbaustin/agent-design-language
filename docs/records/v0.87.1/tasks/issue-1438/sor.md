# [v0.87.1][WP-04] Trace-aligned runtime execution

Task ID: issue-1438
Run ID: issue-1438
Version: v0.87.1
Title: [v0.87.1][WP-04] Trace-aligned runtime execution
Branch: codex/1438-v0-87-1-wp-04-trace-aligned-runtime-execution
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T18:41:00Z
- End Time: 2026-04-08T18:55:21Z

## Summary
Implemented a trace-aligned runtime execution pass for `v0.87.1` and aligned the runtime, learning-export, and review/docs surfaces to the same bounded runtime truth. The branch now records stable trace-linked runtime execution and trace-bundle export/import behavior without claiming broader lifecycle or persistence semantics.

## Artifacts produced
- `adl/src/trace_schema_v1.rs`
- `adl/src/cli/run_artifacts/runtime.rs`
- `adl/src/cli/run_artifacts/summary.rs`
- `adl/src/learning_export/trace_bundle_v2.rs`
- `adl/src/cli/tests/run_state/persistence.rs`
- `adl/src/cli/tests/artifact_builders/summary.rs`
- `adl/src/learning_export/tests.rs`
- `adl/tests/cli_smoke/instrument_and_cli.rs`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`

## Actions taken
- aligned runtime trace schema and runtime-artifact summary generation with the trace-aligned execution model
- added deterministic trace bundle v2 export/import coverage and replay-friendly manifest hashing
- tightened run-state persistence and resume round-trip behavior for trace-backed execution
- updated CLI smoke coverage to exercise instrumented execution and trace-aligned CLI surfaces
- revised the milestone demo matrix and runtime-environment architecture doc to match the actual runtime proof surfaces
- validated the worktree with focused tests, formatting, and lint checks only

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet
- Worktree-only paths remaining:
  - `adl/src/trace_schema_v1.rs`
  - `adl/src/cli/run_artifacts/runtime.rs`
  - `adl/src/cli/run_artifacts/summary.rs`
  - `adl/src/learning_export/trace_bundle_v2.rs`
  - `adl/src/cli/tests/run_state/persistence.rs`
  - `adl/src/cli/tests/artifact_builders/summary.rs`
  - `adl/src/learning_export/tests.rs`
  - `adl/tests/cli_smoke/instrument_and_cli.rs`
  - `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
  - `docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct write in bound issue worktree
- Verification performed:
  - `git status --short` to confirm the branch is still worktree-only
  - `ls`/`sed` path checks to confirm the tracked SOR path and updated artifact surfaces exist
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml write_run_state_and_load_resume_round_trip -- --nocapture` to verify the run-state persistence round trip
  - `cargo test --manifest-path adl/Cargo.toml build_run_summary_sorts_remote_policy_and_tracks_denials -- --nocapture` to verify summary ordering and policy tracking
  - `cargo test --manifest-path adl/Cargo.toml export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match -- --nocapture` to verify deterministic bundle export and manifest hashing
  - `cargo test --manifest-path adl/Cargo.toml import_trace_bundle_v2_accepts_valid_bundle_and_returns_activation_log_path -- --nocapture` to verify bundle import and activation-log handling
  - `cargo test --manifest-path adl/Cargo.toml run_executes_call_workflow_with_namespaced_state_and_trace_events -- --nocapture` to verify trace-aligned runtime execution
  - `cargo test --manifest-path adl/Cargo.toml instrument_replay_bundle_from_trace_bundle_v2_is_stable -- --nocapture` to verify stable replay bundle instrumentation
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` to verify formatting
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` to verify lint cleanliness
- Results:
  - all listed validation commands passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test --manifest-path adl/Cargo.toml write_run_state_and_load_resume_round_trip -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml build_run_summary_sorts_remote_policy_and_tracks_denials -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml import_trace_bundle_v2_accepts_valid_bundle_and_returns_activation_log_path -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml run_executes_call_workflow_with_namespaced_state_and_trace_events -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml instrument_replay_bundle_from_trace_bundle_v2_is_stable -- --nocapture"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
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
  - `cargo test --manifest-path adl/Cargo.toml export_trace_bundle_v2_is_deterministic_and_manifest_hashes_match -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml instrument_replay_bundle_from_trace_bundle_v2_is_stable -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml write_run_state_and_load_resume_round_trip -- --nocapture`
- Fixtures or scripts used:
  - the focused trace-bundle v2 export/import tests and run-state persistence round-trip tests
- Replay verification (same inputs -> same artifacts/order):
  - the trace bundle export and replay path preserved the same manifest hashes and stable replay bundle outputs for fixed inputs
- Ordering guarantees (sorting / tie-break rules used):
  - remote policy sorting and denial tracking are deterministic for a fixed execution state
- Artifact stability notes:
  - the trace bundle and runtime summary surfaces remain stable for the same run inputs and trace material

## Security / Privacy Checks
- Secret leakage scan performed:
  - reviewed the updated trace/runtime/docs surfaces for secrets or tokens; none were introduced
- Prompt / tool argument redaction verified:
  - the validation and artifact text do not record prompts or tool arguments
- Absolute path leakage check:
  - checked the recorded artifact paths and command list for unjustified host-path leakage; none were added
- Sandbox / policy invariants preserved:
  - yes; the change stayed within bounded runtime, trace, export/import, test, and doc surfaces

## Replay Artifacts
- Trace bundle path(s):
  - worktree-local trace bundle v2 fixtures under the focused `learning_export` and CLI smoke tests
- Run artifact root:
  - `.adl/runs/` for runtime artifacts and `trace_bundle_v2/` for replay-bundle export fixtures
- Replay command used for verification:
  - `cargo test --manifest-path adl/Cargo.toml instrument_replay_bundle_from_trace_bundle_v2_is_stable -- --nocapture`
- Replay result:
  - passed with stable trace bundle replay output

## Artifact Verification
- Primary proof surface:
  - `adl/src/trace_schema_v1.rs`, `adl/src/learning_export/trace_bundle_v2.rs`, and the updated runtime/summary test surfaces
- Required artifacts present:
  - yes
- Artifact schema/version checks:
  - the trace bundle v2 export/import path and runtime trace schema updates passed focused tests
- Hash/byte-stability checks:
  - manifest hash matching passed for deterministic export
- Missing/optional artifacts and rationale:
  - no standalone demo was added here; this issue is about runtime/trace alignment and proof surfaces, not a new demo tranche

## Decisions / Deviations
- kept the closeout record pre-PR-open and truthful to the current worktree-only state
- recorded the actual validation suite rather than the broader repository test matrix
- did not invent tracked main-repo integration because the branch is not published yet

## Follow-ups / Deferred work
- merge publication and post-PR cleanup remain for later workflow phases
- once the PR opens, the tracked review surfaces can be used for janitor and closeout truth reconciliation
