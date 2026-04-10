# v0-87-1-runtime-add-trace-run-manifest-and-provenance-capture

Task ID: issue-1519
Run ID: issue-1519
Version: v0.87.1
Title: [v0.87.1][runtime] Add trace run manifest and provenance capture
Branch: codex/1519-v0-87-1-runtime-add-trace-run-manifest-and-provenance-capture
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5.4
- Provider: ChatGPT
- Start Time: 2026-04-09T18:52:00-07:00
- End Time: 2026-04-09T19:06:00-07:00

## Summary

Implemented the trace/run provenance and archive/discovery work for #1519, #1520, and #1521 in one coordinated branch because the runtime manifest, provider-demo archive routing, and learning-export discovery surfaces share the same run-artifact contract.

## Artifacts produced
- Runtime now writes `run_manifest.json` beside each new run's `run.json`, `steps.json`, `run_status.json`, and `run_summary.json`.
- Learning export now discovers both flat `.adl/runs/<run_id>` roots and milestone-organized `.adl/trace-archive/milestones/<milestone>/runs/<run_id>` roots.
- Provider demo wrappers now pass `ADL_MILESTONE=v0.87.1`, persist demo names into manifests, run the archive helper after successful execution, and print the canonical archive path.
- Archive manifests now record whether `run_manifest.json` exists and prefer manifest milestone metadata when available.
- v0.87.1 runtime/demo docs now describe `run_manifest.json`, `.adl/trace-archive`, and archive-aware learning export.

## Actions taken
- Added a `run_manifest.json` path accessor and runtime manifest writer.
- Added privacy-safe manifest fields for run id, workflow id, ADL version, explicit milestone, issue/PR environment hints, demo name, provider ids, runtime-root source classes, trace status, and generated artifact names.
- Updated learning export internals to resolve run references by concrete directory rather than assuming every run lives directly under one flat root.
- Updated trace-bundle export to include `run_manifest.json` when present while preserving compatibility with historical runs that do not have it.
- Updated provider demo common tooling and current provider demos to copy/index bounded runtime outputs into `.adl/trace-archive`.
- Updated targeted tests and docs.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked code, shell helpers, tests, and docs on the issue branch.
- Worktree-only paths remaining: ignored local demo output under a temporary output directory and ignored worktree `.adl/trace-archive` smoke output only.
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: issue worktree branch plus `pr finish` publication.
- Verification performed:
  - `git status --short`
  - `git diff --stat`
  - targeted Rust, shell, clippy, and mock-provider demo checks listed below
- Result: PASS

## Validation
- `bash -n adl/tools/archive_run_artifacts.sh adl/tools/provider_demo_common.sh adl/tools/demo_v0871_provider_mock.sh adl/tools/demo_v0871_provider_http.sh adl/tools/demo_v0871_provider_local_ollama.sh adl/tools/demo_v0871_provider_chatgpt.sh adl/tools/test_archive_run_artifacts.sh`: verified edited shell scripts parse.
- `cargo fmt --manifest-path adl/Cargo.toml --check`: verified Rust formatting.
- `bash adl/tools/test_archive_run_artifacts.sh`: verified archive inventory/apply behavior, manifest milestone inference, and copied archive source records.
- `bash adl/tools/test_provider_demo_common.sh`: verified provider demo README helper behavior after adding archive helper functions.
- `cargo test --manifest-path adl/Cargo.toml learning_export -- --nocapture`: verified archive-aware discovery/export behavior and duplicate run-id handling.
- `cargo test --manifest-path adl/Cargo.toml run_state -- --nocapture`: verified runtime artifact writing, including `run_manifest.json` and privacy-safe override-root recording.
- `bash adl/tools/demo_v0871_provider_mock.sh <temp-demo-output>`: verified local/mock provider demo creates a run and archives it under `.adl/trace-archive/milestones/v0.87.1/runs/<run_id>`.
- `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`: verified clippy cleanliness for all targets.
- Results: PASS

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo fmt --manifest-path adl/Cargo.toml --check"
      - "cargo test --manifest-path adl/Cargo.toml learning_export -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml run_state -- --nocapture"
      - "cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"
      - "bash adl/tools/test_archive_run_artifacts.sh"
      - "bash adl/tools/test_provider_demo_common.sh"
      - "bash adl/tools/demo_v0871_provider_mock.sh <temp-demo-output>"
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
      approved: true
```

## Determinism Evidence
- Determinism tests executed: learning export fixtures verify stable export ordering and duplicate run-id behavior.
- Fixtures or scripts used: `learning_export` unit tests and `test_archive_run_artifacts.sh`.
- Replay verification: mock provider demo reruns locally without network credentials and produces the expected proof surfaces.
- Ordering guarantees: run references sort by run id and then path; duplicate run ids across archive buckets are reported clearly instead of silently choosing a source.
- Artifact stability notes: trace-bundle export still accepts historical runs without `run_manifest.json`; new runs include the manifest when present.

## Security / Privacy Checks
- Secret leakage scan performed: manifest fields intentionally store source categories and relative/generated artifact names rather than host-specific absolute paths.
- Prompt / tool argument redaction verified: validation does not persist provider tokens; local ChatGPT/HTTP scripts use bounded local tokens only in process environment and logs already scoped to demo output.
- Absolute path leakage check: run-state test verifies override-root manifest output does not contain the absolute runs-root path.
- Sandbox / policy invariants preserved: run id validation remains path-segment based, and learning export rejects unsafe explicit run ids.

## Replay Artifacts
- Trace bundle path(s): no tracked trace bundle was generated; local mock demo proof was archived in the ignored worktree `.adl/trace-archive`.
- Run artifact root: `.adl/trace-archive/milestones/v0.87.1/runs/v0-87-1-provider-mock-demo` in the issue worktree during validation.
- Replay command used for verification: `bash adl/tools/demo_v0871_provider_mock.sh <temp-demo-output>`.
- Replay result: PASS

## Artifact Verification
- Primary proof surface: `run_manifest.json` beside new run artifacts and `.adl/trace-archive/MANIFEST.tsv` for archive inventory.
- Required artifacts present: PASS for runtime manifest, archive manifest, provider demo archive output, and learning export fixture outputs.
- Artifact schema/version checks: `trace_run_manifest.v1` emitted for new runtime runs.
- Hash/byte-stability checks: existing bundle/trace-bundle manifest hash tests remain passing.
- Missing/optional artifacts and rationale: historical runs are not required to have `run_manifest.json`; trace-bundle export includes it only when present.

## Decisions / Deviations
- #1519, #1520, and #1521 were implemented together because separate PRs would overlap the same runtime/export/archive surfaces and create avoidable merge conflict churn.
- Doctor preflight was overridden because existing draft PR #1517 is still open; this was intentional and recorded rather than hidden.

## Follow-ups / Deferred work
- #1473 still owns the final local `.adl/runs` active-root cleanup/retention pass after this preservation/discovery work lands.
