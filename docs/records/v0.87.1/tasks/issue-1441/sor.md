# [v0.87.1][WP-07] Runtime state / persistence discipline

Task ID: issue-1441
Run ID: issue-1441
Version: v0.87.1
Title: [v0.87.1][WP-07] Runtime state / persistence discipline
Branch: codex/1441-v0-87-1-wp-07-runtime-state-persistence-discipline
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5
- Provider: OpenAI
- Start Time: 2026-04-08T20:30:00Z
- End Time: 2026-04-08T21:08:24Z

## Summary

Implemented the bounded WP-07 persistence contract by making `run_status.json` explicitly describe persistence mode, cleanup disposition, resume guard, and the authoritative state-artifact inventory. Added a canonical D7 demo/test pair for paused-vs-completed runtime state, and aligned the lifecycle, resilience, Shepherd, and demo-matrix docs to the shipped persistence surface.

## Artifacts produced

- persisted `run_status.json` fields for persistence discipline
- `adl/tools/demo_v0871_runtime_state.sh`
- `adl/tools/test_demo_v0871_runtime_state.sh`
- updated WP-07 feature and demo docs

## Actions taken

- extended `RunStatusArtifact` with explicit persistence-discipline fields
- derived bounded persistence and cleanup semantics in `build_run_status`
- added focused Rust tests for paused, failed, and completed persistence states
- added a runnable D7 runtime-state demo and regression test
- updated milestone docs to point at the real persisted state contract

## Main Repo Integration (REQUIRED)

- Main-repo paths updated: none yet
- Worktree-only paths remaining:
  - `adl/src/cli/run_artifacts/summary.rs`
  - `adl/src/cli/run_artifacts_types.rs`
  - `adl/src/cli/tests/artifact_builders/learning_runtime.rs`
  - `adl/src/cli/tests/run_state/persistence.rs`
  - `adl/tools/demo_v0871_runtime_state.sh`
  - `adl/tools/test_demo_v0871_runtime_state.sh`
  - `adl/tools/README.md`
  - `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
  - `docs/milestones/v0.87.1/features/AGENT_LIFECYCLE.md`
  - `docs/milestones/v0.87.1/features/LOCAL_RUNTIME_RESILIENCE.md`
  - `docs/milestones/v0.87.1/features/SHEPHERD_RUNTIME_MODEL.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: bounded worktree update pending `pr finish`
- Verification performed:
  - `git diff --check`
  - `bash adl/tools/test_demo_v0871_runtime_state.sh`
  - `cargo test --manifest-path adl/Cargo.toml build_run_status -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts -- --nocapture`
  - `cargo test --manifest-path adl/Cargo.toml load_resume_state_rejects_non_paused_status -- --nocapture`
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
  - `cargo test --manifest-path adl/Cargo.toml`
- Result: PASS

## Validation

- `git diff --check` verified the worktree patch shape is clean.
- `bash adl/tools/test_demo_v0871_runtime_state.sh` verified the D7 demo writes the expected paused/completed state artifacts and README proof surface.
- `cargo test --manifest-path adl/Cargo.toml build_run_status -- --nocapture` verified the new persistence-discipline fields for paused, failed, and corruption paths.
- `cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts -- --nocapture` verified runtime-state artifact emission still works.
- `cargo test --manifest-path adl/Cargo.toml load_resume_state_rejects_non_paused_status -- --nocapture` verified completed runs do not masquerade as resumable state.
- `cargo fmt --manifest-path adl/Cargo.toml --all --check` verified formatting.
- `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` verified lint cleanliness.
- `cargo test --manifest-path adl/Cargo.toml` verified the full crate suite after the WP-07 changes.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "git diff --check"
      - "bash adl/tools/test_demo_v0871_runtime_state.sh"
      - "cargo test --manifest-path adl/Cargo.toml build_run_status -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml write_run_state_artifacts -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml load_resume_state_rejects_non_paused_status -- --nocapture"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"
      - "cargo test --manifest-path adl/Cargo.toml"
  determinism:
    status: PARTIAL
    replay_verified: false
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
      approved: true
```

## Determinism Evidence

- Determinism tests executed:
  - `cargo test --manifest-path adl/Cargo.toml build_run_status -- --nocapture`
  - `bash adl/tools/test_demo_v0871_runtime_state.sh`
  - `cargo test --manifest-path adl/Cargo.toml`
- Fixtures or scripts used:
  - `adl/examples/v0-6-hitl-pause-resume.adl.yaml`
  - `adl/examples/v0-6-hitl-no-pause.adl.yaml`
  - `adl/tools/mock_ollama_v0_4.sh`
- Replay verification: not fully proven; this issue establishes stable persistence classification and artifact inventory, but it does not add a byte-for-byte repeated-run replay fixture.
- Ordering guarantees: the persisted `state_artifacts` inventory is emitted in a fixed deterministic order, and the full crate suite continues to cover deterministic run-status artifact behavior.
- Artifact stability notes: paused and completed runs now expose different but explicit persistence contracts without ambiguous cleanup state.

## Security / Privacy Checks

- Secret leakage scan performed: not separately needed; the changes are code, docs, and bounded artifact-path metadata only.
- Prompt / tool argument redaction verified: the new persisted fields expose only lifecycle/persistence metadata and artifact names, not prompts or tool arguments.
- Absolute path leakage check: passed; recorded commands and artifact references remain repository-relative.
- Sandbox / policy invariants preserved: yes; the work stayed within the managed worktree and existing local validation surfaces.

## Replay Artifacts

- Trace bundle path(s): none
- Run artifact root: `artifacts/v0871/runtime_state/runtime/runs/`
- Replay command used for verification: none
- Replay result: not applicable for this bounded persistence-discipline slice

## Artifact Verification

- Primary proof surface: `artifacts/v0871/runtime_state/runtime/runs/v0-6-hitl-pause-demo/run_status.json`
- Required artifacts present:
  - paused `run_status.json`
  - paused `pause_state.json`
  - completed `run_status.json`
  - paused/completed `logs/trace_v1.json`
  - runtime `runtime_environment.json`
- Artifact schema/version checks: `RunStatusArtifact` schema was extended intentionally and covered by targeted tests plus the full crate suite.
- Hash/byte-stability checks: no dedicated hash check added in this issue.
- Missing/optional artifacts and rationale: replay bundles are not part of the bounded WP-07 proof surface.

## Decisions / Deviations

- Kept determinism at `PARTIAL` because this issue did not add a full repeated-run replay fixture.
- Marked schema changes as approved because WP-07 intentionally extends the persisted `run_status.json` contract.
- Used the D7 demo as the primary reviewer surface rather than introducing a broader integrated runtime demo early.

## Follow-ups / Deferred work

- Publish through `pr finish` and tracked SOR sync.
- Use WP-08 to build the higher-level reviewer entry surface over the new runtime-state artifacts.
