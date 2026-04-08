# [v0.87.1][WP-05] Local runtime resilience + Shepherd preservation

Task ID: issue-1439
Run ID: issue-1439
Version: v0.87.1
Title: [v0.87.1][WP-05] Local runtime resilience + Shepherd preservation
Branch: codex/1439-v0-87-1-wp-05-local-runtime-resilience-shepherd-preservation
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T19:21:15Z
- End Time: 2026-04-08T20:01:09Z

## Summary

Implemented an explicit local resilience and Shepherd preservation contract by extending `run_status.json` with inspectable preservation/continuity fields, proving resumable interruption and corruption-refusal behavior in tests, and aligning the WP-05 feature docs and demo matrix to those concrete artifact surfaces.

## Artifacts produced
- `adl/src/cli/run_artifacts_types.rs`
- `adl/src/cli/run_artifacts/summary.rs`
- `adl/src/cli/run_artifacts/runtime.rs`
- `adl/src/cli/tests/artifact_builders/learning_runtime.rs`
- `adl/src/cli/tests/run_state/persistence.rs`
- `docs/milestones/v0.87.1/features/LOCAL_RUNTIME_RESILIENCE.md`
- `docs/milestones/v0.87.1/features/SHEPHERD_RUNTIME_MODEL.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`

## Actions taken
- Added explicit local resilience fields to `run_status.json`: `resilience_classification`, `continuity_status`, `preservation_status`, and `shepherd_decision`.
- Derived those fields from bounded local evidence: pause/resume state, failure classification, and replay-invariant corruption.
- Updated the runtime writer so the new resilience fields are emitted with the canonical run-status artifact.
- Added focused unit and run-state tests for:
  - resumable interruption / preserved pause state
  - replay-invariant corruption / refuse-resume behavior
  - persisted paused-run artifact truth
- Updated the WP-05 docs and demo matrix so they point at the concrete `run_status.json`, `pause_state.json`, and `logs/trace_v1.json` proof surfaces.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining:
  - `adl/src/cli/run_artifacts_types.rs`
  - `adl/src/cli/run_artifacts/summary.rs`
  - `adl/src/cli/run_artifacts/runtime.rs`
  - `adl/src/cli/tests/artifact_builders/learning_runtime.rs`
  - `adl/src/cli/tests/run_state/persistence.rs`
  - `docs/milestones/v0.87.1/features/LOCAL_RUNTIME_RESILIENCE.md`
  - `docs/milestones/v0.87.1/features/SHEPHERD_RUNTIME_MODEL.md`
  - `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct edits in the bound issue worktree pending `pr finish` publication
- Verification performed:
  - `git status --short`
  - targeted file reads of the changed code and docs surfaces
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml build_run_status_tracks_attempts_and_resume_completed_steps -- --nocapture`
    - verified the run-status artifact now carries explicit resilience / Shepherd fields for failure-review cases
  - `cargo test --manifest-path adl/Cargo.toml build_run_status_marks_paused_runs_as_resumable_interruption -- --nocapture`
    - verified paused runs emit resumable interruption / preserved pause-state classification
  - `cargo test --manifest-path adl/Cargo.toml build_run_status_refuses_resume_for_replay_invariant_corruption -- --nocapture`
    - verified replay-invariant corruption is classified as continuity refusal with a refuse-resume Shepherd decision
  - `cargo test --manifest-path adl/Cargo.toml write_run_state_and_load_resume_round_trip -- --nocapture`
    - verified persisted paused-run artifacts include the new resilience / preservation truth in `run_status.json`
  - `cargo fmt --manifest-path adl/Cargo.toml --all`
    - normalized Rust formatting after the artifact and test changes
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
    - verified the WP-05 code changes are warning-free across all targets
- Results:
  - all listed validation commands passed
  - the branch was ready to enter the finish publication wave after the SOR was normalized

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "targeted run-status resilience builder tests"
      - "paused-run persistence round-trip test"
      - "cargo clippy --all-targets -- -D warnings"
  determinism:
    status: PARTIAL
    replay_verified: false
    ordering_guarantees_verified: false
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
  - `build_run_status_tracks_attempts_and_resume_completed_steps`
  - `build_run_status_marks_paused_runs_as_resumable_interruption`
  - `build_run_status_refuses_resume_for_replay_invariant_corruption`
- Fixtures or scripts used:
  - deterministic bad-trace fixture written in the corruption test
  - deterministic paused-run round-trip in the persistence test
- Replay verification (same inputs -> same artifacts/order):
  - not a full replay-bundle issue; this work proves stable resilience classification for fixed local inputs rather than end-to-end replay equivalence
- Ordering guarantees (sorting / tie-break rules used):
  - not the primary claim for WP-05
- Artifact stability notes:
  - the new run-status resilience fields are derived from bounded local evidence and tested with fixed expected values

## Security / Privacy Checks
- Secret leakage scan performed:
  - no secrets were added; the new surfaces only expose bounded runtime classification strings
- Prompt / tool argument redaction verified:
  - the new WP-05 fields do not record prompts or tool arguments
- Absolute path leakage check:
  - recorded paths in this SOR are repository-relative
- Sandbox / policy invariants preserved:
  - yes; the change does not widen runtime permissions or bypass existing policy classification

## Replay Artifacts
- Trace bundle path(s): not applicable for this issue's direct proof surface
- Run artifact root: bounded test run directories under the existing `.adl/runs/<run_id>/` model
- Replay command used for verification: not applicable
- Replay result: WP-05 proves pause/resume preservation and corruption refusal behavior rather than full replay-bundle equivalence

## Artifact Verification
- Primary proof surface:
  - `run_status.json` resilience / continuity / Shepherd decision fields, with `pause_state.json` and `logs/trace_v1.json` as supporting runtime artifacts
- Required artifacts present:
  - yes in the tested paused-run artifact set
- Artifact schema/version checks:
  - the `RunStatusArtifact` schema was extended in code and validated through compile-time serde usage plus runtime test assertions
- Hash/byte-stability checks:
  - not applicable; this issue focused on explicit classification truth rather than byte-for-byte export equality
- Missing/optional artifacts and rationale:
  - no standalone new Shepherd daemon or checkpoint artifact was added; WP-05 intentionally uses the existing bounded run artifact model

## Decisions / Deviations
- Implemented the Shepherd preservation contract as explicit bounded fields in `run_status.json` instead of inventing a separate runtime service.
- Kept the scope local-first: interruption, crash-review, and corruption-refusal are explicit; distributed continuity and richer identity systems remain out of scope.

## Follow-ups / Deferred work
- `WP-07` should build on these preservation fields when tightening runtime state and persistence discipline.
- Planned demos `D4`, `D4A`, and `D5` still need their runnable entrypoints, but the concrete proof surfaces are now defined.
