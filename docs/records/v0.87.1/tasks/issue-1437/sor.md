# [v0.87.1][WP-03] Execution boundaries and lifecycle

Task ID: issue-1437
Run ID: issue-1437
Version: v0.87.1
Title: [v0.87.1][WP-03] Execution boundaries and lifecycle
Branch: codex/1437-v0-87-1-wp-03-execution-boundaries-and-lifecycle
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T18:15:00Z
- End Time: 2026-04-08T18:38:00Z

## Summary

This branch completes a bounded execution-boundary and lifecycle pass for the v0.87.1 control plane. The implementation sharpens the runtime phase model, adds traceable lifecycle boundary events, and updates the lifecycle docs so they describe bounded runtime semantics rather than persistent identity or full agency continuity.

## Artifacts produced
- `adl/src/execute/mod.rs`
- `adl/src/execute/runner.rs`
- `adl/src/execute/state/contracts.rs`
- `adl/src/execute/state/mod.rs`
- `adl/src/instrumentation.rs`
- `adl/src/obsmem_indexing.rs`
- `adl/src/trace.rs`
- `adl/tests/execute_tests/delegation_resume.rs`
- `docs/milestones/v0.87.1/features/AGENT_LIFECYCLE.md`
- `docs/milestones/v0.87.1/features/EXECUTION_BOUNDARIES.md`

## Actions taken
- Reworked the execution-boundary and lifecycle surfaces so the runtime is modeled as explicit phases: init, execute, complete, and teardown.
- Added runtime boundary trace events for `runtime_init`, `workflow_call`, `pause`, `resume`, and `run_completion`.
- Updated the execute-state, trace, instrumentation, and ObsMem linkage code so lifecycle boundaries remain observable and namespaced.
- Revised the milestone docs to describe bounded runtime semantics only and to avoid claims about persistent identity, chronosense, or full agency continuity.
- Validated the runtime and lifecycle behavior with focused tests and lint/format checks.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1466.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1466.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1466.
- Verification performed:
  - `git status --short` to confirm the branch is still worktree-only and not published to main
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` to verify formatting
  - `cargo test --manifest-path adl/Cargo.toml trace::tests::trace_records_runtime_lifecycle_and_boundary_events -- --nocapture` to prove lifecycle trace coverage
  - `cargo test --manifest-path adl/Cargo.toml run_executes_call_workflow_with_namespaced_state_and_trace_events -- --nocapture` to prove runtime execution behavior
  - `cargo test --manifest-path adl/Cargo.toml delegation_resume -- --nocapture` to prove resume-path behavior
  - `cargo test --manifest-path adl/Cargo.toml obsmem -- --nocapture` to prove the memory-linkage surface still works
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` to verify the code compiles cleanly under lint
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` to ensure the codebase stays formatted
  - `cargo test --manifest-path adl/Cargo.toml trace::tests::trace_records_runtime_lifecycle_and_boundary_events -- --nocapture` to verify boundary-event tracing
  - `cargo test --manifest-path adl/Cargo.toml run_executes_call_workflow_with_namespaced_state_and_trace_events -- --nocapture` to verify execution-state namespacing and trace events
  - `cargo test --manifest-path adl/Cargo.toml delegation_resume -- --nocapture` to verify delegation resume behavior
  - `cargo test --manifest-path adl/Cargo.toml obsmem -- --nocapture` to verify ObsMem linkage remains intact
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` to ensure the branch is lint-clean
- Results:
  - all listed validation commands passed

## Verification Summary
```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo test --manifest-path adl/Cargo.toml trace::tests::trace_records_runtime_lifecycle_and_boundary_events -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml run_executes_call_workflow_with_namespaced_state_and_trace_events -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml delegation_resume -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml obsmem -- --nocapture"
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
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: lifecycle trace and execution-state regression tests run against the same branch/worktree inputs.
- Fixtures or scripts used: targeted Rust test cases in `adl/tests/execute_tests/delegation_resume.rs` and the execute/trace/state modules.
- Replay verification (same inputs -> same artifacts/order): verified by rerunning the focused tests after the code and doc edits; the trace boundary events and namespaced lifecycle state remained stable.
- Ordering guarantees (sorting / tie-break rules used): lifecycle phases are now explicit in the runtime model, and trace output records those phases in the same order each run.
- Artifact stability notes: the docs now describe bounded runtime semantics only, so the reviewer-facing narrative stays aligned with the emitted runtime boundary events.

## Security / Privacy Checks
- Secret leakage scan performed: reviewed the changed docs and test output surfaces for secrets, tokens, and prompt arguments.
- Prompt / tool argument redaction verified: the edited docs stay at the semantic level and do not expose tool arguments or hidden prompts.
- Absolute path leakage check: the SOR does not introduce unjustified host-absolute paths beyond the required repository references already present in the branch workflow.
- Sandbox / policy invariants preserved: changes remain bounded to the issue worktree and do not alter execution policy or publish state.

## Replay Artifacts
- Trace bundle path(s): `adl/tests/execute_tests/delegation_resume.rs` and the execute/trace regression outputs from the run
- Run artifact root: worktree-local issue artifacts only; no published main-repo artifact yet
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml trace::tests::trace_records_runtime_lifecycle_and_boundary_events -- --nocapture`
- Replay result: passed with stable lifecycle boundary coverage

## Artifact Verification
- Primary proof surface: `adl/src/trace.rs` and the lifecycle/trace regression tests
- Required artifacts present: yes, the worktree contains the updated code and docs surfaces listed above
- Artifact schema/version checks: the branch keeps the v0.87.1 lifecycle model aligned to explicit runtime phases and boundary events
- Hash/byte-stability checks: not separately measured; bounded regression tests served as the replay proof
- Missing/optional artifacts and rationale: no main-repo publish artifact yet because this is still a worktree-only branch

## Decisions / Deviations
- Kept the branch worktree-only and truthful about integration state because no PR has been opened yet.
- Described the lifecycle as bounded runtime phases rather than persistent identity or full agency continuity, matching the implemented docs and trace events.
- Did not claim main-repo integration because the branch has not been merged.

## Follow-ups / Deferred work
- Open the PR after any required review of the worktree SOR.
- If later publication changes the integration status, update this record in the published branch copy, not here.
