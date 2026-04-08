# [v0.87.1][WP-08] Runtime review surfaces

Task ID: issue-1442
Run ID: issue-1442
Version: v0.87.1
Title: [v0.87.1][WP-08] Runtime review surfaces
Branch: codex/1442-v0-87-1-wp-08-runtime-review-surfaces
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T21:25:08Z
- End Time: 2026-04-08T21:36:51Z

## Summary
Added one bounded `v0.87.1` runtime review walkthrough package and one deterministic Rust validator so reviewers can move from the milestone docs to the D6 operator proof surface and D7 runtime-state proof surface without reconstructing artifact roots by hand. The tooling, demo matrix, and reviewer docs now all point at the same D8 manifest-and-README contract.

## Artifacts produced
- `adl/src/cli/tooling_cmd.rs`
- `adl/src/cli/tooling_cmd/review_surface.rs`
- `adl/src/cli/tooling_cmd/tests.rs`
- `adl/src/cli/usage.rs`
- `adl/tools/demo_v0871_review_surface.sh`
- `adl/tools/test_demo_v0871_review_surface.sh`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md`
- `docs/tooling/README.md`
- `docs/tooling/review-surface-format.md`
- `docs/tooling/reviewer-surface.md`

## Actions taken
- added `adl tooling review-runtime-surface --review-root <dir>` as the deterministic validator for the D8 runtime review package
- added `adl/tools/demo_v0871_review_surface.sh` to assemble the bounded review package from the existing D6 operator and D7 runtime-state proof roots
- added `adl/tools/test_demo_v0871_review_surface.sh` to build the D8 package and validate it end to end
- extended tooling tests to cover the new runtime-review validator and dispatch surface
- marked D8 as `READY` in the milestone demo matrix with the real command, manifest, README, and proof-surface mapping
- aligned the runtime feature index and tooling docs so the review-surface vocabulary matches the implemented D8 contract

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: pr_branch
- Integration method used: bounded branch update published via `pr finish` and opened as draft PR `#1506`
- Verification performed:
  - `git diff --check`
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  - `cargo test --manifest-path adl/Cargo.toml tooling_cmd -- --nocapture`
  - `bash adl/tools/test_demo_v0871_review_surface.sh`
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
  - `cargo test --manifest-path adl/Cargo.toml`
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `git diff --check` to verify the bounded WP-08 patch set is free of whitespace and patch-format defects
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` to verify the new tooling code and docs remain formatting-clean
  - `cargo test --manifest-path adl/Cargo.toml tooling_cmd -- --nocapture` to verify the new runtime-review validator, dispatch surface, and fixture-backed tooling tests
  - `bash adl/tools/test_demo_v0871_review_surface.sh` to verify the D8 review package assembles the documented manifest and README, then validates the package through the new Rust review-surface command
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` to verify the new tooling command and tests remain warning-free
  - `cargo test --manifest-path adl/Cargo.toml` to verify the full crate suite still passes with the new D8 runtime review surface
- Results:
  - diff hygiene check passed
  - formatting check passed
  - tooling command test slice passed
  - D8 shell walkthrough regression passed
  - clippy passed with `-D warnings`
  - full crate suite passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "git diff --check"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo test --manifest-path adl/Cargo.toml tooling_cmd -- --nocapture"
      - "bash adl/tools/test_demo_v0871_review_surface.sh"
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
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed:
  - `cargo test --manifest-path adl/Cargo.toml tooling_cmd -- --nocapture`
  - `bash adl/tools/test_demo_v0871_review_surface.sh`
- Fixtures or scripts used:
  - `adl/tools/demo_v0871_review_surface.sh`
  - `adl/tools/demo_v0871_operator_surface.sh`
  - `adl/tools/demo_v0871_runtime_state.sh`
  - `adl/tools/test_demo_v0871_review_surface.sh`
- Replay verification (same inputs -> same artifacts/order):
  - not claimed as a full repeated-run replay proof; the shell walkthrough regression verifies that the D8 manifest and README assemble correctly and that the referenced proof surfaces resolve for the bounded runtime review package
- Ordering guarantees (sorting / tie-break rules used):
  - the Rust validator enforces one canonical D6-then-D7 package ordering, one canonical manifest identity, and one stable reviewer walkthrough
- Artifact stability notes:
  - the D8 fixture requires the same manifest name, README name, and referenced D6/D7 proof-surface paths to remain stable

## Security / Privacy Checks
- Secret leakage scan performed:
  - manual review of the new scripts, manifest fields, and docs confirmed no secrets or provider tokens were added to the review package surface
- Prompt / tool argument redaction verified:
  - yes; the runtime review package points to proof artifacts and docs only, not raw prompts or raw tool arguments
- Absolute path leakage check:
  - passed; the validator and docs require repository-relative references only, and the shell fixture confirms the assembled package stays path-relative
- Sandbox / policy invariants preserved:
  - yes; the work remained bounded to tooling/docs/tests and used no destructive commands

## Replay Artifacts
- Trace bundle path(s): none
- Run artifact root: `artifacts/v0871/review_surface`
- Replay command used for verification: `bash adl/tools/test_demo_v0871_review_surface.sh`
- Replay result: bounded package assembly and proof-surface verification passed; full repeated-run byte-for-byte replay was not claimed

## Artifact Verification
- Primary proof surface: `artifacts/v0871/review_surface/demo_manifest.json`
- Required artifacts present: yes
- Artifact schema/version checks: the new manifest is validated as `adl.runtime_review_surface.v1`, and the issue reused the existing D6/D7 runtime artifact schemas without changing them
- Hash/byte-stability checks: not run
- Missing/optional artifacts and rationale: no trace bundle or release-review package is required for this bounded Sprint 1 runtime-review-surface issue

## Decisions / Deviations
- Kept WP-08 bounded to one reviewer entry command, one manifest, one README, and one deterministic validator instead of widening into a broader external-review or release-package workflow.
- Reused the already-landed D6 and D7 proof roots rather than inventing a third competing runtime review layout.

## Follow-ups / Deferred work
- later integrated-runtime and review-tail issues should reuse the D8 manifest-and-README contract rather than inventing alternate reviewer entrypoints
- if future runtime review variants expand beyond D6 and D7, the `review-runtime-surface` validator should be extended deliberately rather than relaxed informally
