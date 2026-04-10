# [v0.87.1][WP-06] Operator surfaces

Task ID: issue-1440
Run ID: issue-1440
Version: v0.87.1
Title: [v0.87.1][WP-06] Operator surfaces
Branch: codex/1440-v0-87-1-wp-06-operator-surfaces
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T20:30:00Z
- End Time: 2026-04-08T20:39:27Z

## Summary
Added one canonical `v0.87.1` operator-surface demo wrapper and regression test, then aligned the milestone and tooling docs to that same runtime-operator contract. The resulting operator path is now: one bounded runtime invocation model, one runtime marker surface, and one canonical per-run proof set for operator and reviewer inspection.

## Artifacts produced
- `adl/tools/demo_v0871_operator_surface.sh`
- `adl/tools/test_demo_v0871_operator_surface.sh`
- `adl/tools/README.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/features/ADL_RUNTIME_ENVIRONMENT.md`
- `docs/milestones/v0.87.1/features/EXECUTION_BOUNDARIES.md`
- `docs/tooling/README.md`

## Actions taken
- added the canonical WP-06 demo wrapper for runtime operator bring-up and proof-surface inspection
- added a focused shell regression that verifies the runtime marker, run summary, run status, trace, and README proof surfaces
- documented the operator contract in the runtime-environment and execution-boundaries feature docs
- marked D6 as `READY` in the demo matrix with the concrete command and proof surface
- refreshed the tooling docs so they point to the current milestone status and operator-surface entrypoint

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1484.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1484.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1484.
- Verification performed:
  - `git diff --check`
  - `bash adl/tools/test_demo_v0871_operator_surface.sh`
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `git diff --check` to verify there are no whitespace or patch-format problems in the bounded operator-surface change set
  - `bash adl/tools/test_demo_v0871_operator_surface.sh` to verify the new operator-surface demo produces the documented runtime marker, run summary, run status, trace artifact, and README proof surfaces
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` to verify the repo remains formatting-clean after the shell/doc updates
- Results:
  - bounded diff check passed
  - operator-surface shell regression passed
  - formatting check passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "git diff --check"
      - "bash adl/tools/test_demo_v0871_operator_surface.sh"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
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
  - `bash adl/tools/test_demo_v0871_operator_surface.sh`
- Fixtures or scripts used:
  - `adl/tools/demo_v0871_operator_surface.sh`
  - `adl/tools/mock_ollama_v0_4.sh`
- Replay verification (same inputs -> same artifacts/order):
  - not fully replay-verified; the focused shell regression proves the documented operator proof surfaces are emitted correctly for a fixed bounded scenario
- Ordering guarantees (sorting / tie-break rules used):
  - the operator contract fixes one canonical command path and one canonical proof-surface set for the bounded runtime demo
- Artifact stability notes:
  - the shell regression requires the same named proof surfaces to appear under the documented runtime root

## Security / Privacy Checks
- Secret leakage scan performed:
  - manual review of the new script and docs surfaces; no secret-bearing inputs or tokens were added
- Prompt / tool argument redaction verified:
  - yes; the documented proof surfaces do not record prompts or raw tool arguments
- Absolute path leakage check:
  - passed; the committed docs and scripts use repository-relative paths and commands
- Sandbox / policy invariants preserved:
  - yes; no destructive commands or out-of-scope file edits were used

## Replay Artifacts
- Trace bundle path(s): none
- Run artifact root: `artifacts/v0871/operator_surface/runtime/runs/v0-4-demo-deterministic-replay`
- Replay command used for verification: `bash adl/tools/test_demo_v0871_operator_surface.sh`
- Replay result: bounded artifact-shape verification passed, but full repeated-run replay was not claimed

## Artifact Verification
- Primary proof surface: `artifacts/v0871/operator_surface/runtime/runs/v0-4-demo-deterministic-replay/run_summary.json`
- Required artifacts present: yes
- Artifact schema/version checks: existing runtime marker and run artifact schemas were reused without schema changes
- Hash/byte-stability checks: not run
- Missing/optional artifacts and rationale: no additional replay bundle or coverage artifact is required for this bounded operator-surface issue

## Decisions / Deviations
- Kept the implementation bounded to docs and shell surfaces instead of widening into a broader runtime-command refactor.
- Chose one canonical demo wrapper and test to establish the operator contract for later Sprint 1 work.

## Follow-ups / Deferred work
- later Sprint 1 demo and review issues should inherit the D6 operator contract rather than inventing new runtime proof layouts
