# [v0.87.1][tools] Ensure SOR is finalized before PR open and published on a tracked review surface

Task ID: issue-1449
Run ID: issue-1449
Version: v0.87.1
Title: [v0.87.1][tools] Ensure SOR is finalized before PR open and published on a tracked review surface
Branch: codex/1449-v0-87-1-tools-ensure-sor-is-finalized-before-pr-open-and-published-on-a-tracked-review-surface
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T17:00:00Z
- End Time: 2026-04-08T17:33:53Z

## Summary
Tightened the finish workflow so a completed SOR is published onto a tracked review surface under `docs/records/<scope>/tasks/issue-<padded>/sor.md` before PR publication, while preserving the local `.adl` task-bundle and `.adl/cards` compatibility mirrors. The finish path now stages that tracked SOR automatically, and the finish docs/contracts/tests teach the corrected SOR-before-open behavior.

## Artifacts produced
- `adl/src/cli/pr_cmd.rs`
- `adl/src/control_plane.rs`
- `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- `adl/tools/card_paths.sh`
- `adl/tools/test_pr_finish_default_stage_root.sh`
- `adl/tools/skills/pr-finish/SKILL.md`
- `adl/tools/skills/pr-finish/references/output-contract.md`
- `docs/default_workflow.md`
- `docs/records/README.md`

## Actions taken
- added control-plane helpers for the tracked public SOR path under `docs/records/<scope>/tasks/issue-<padded>/`
- updated `real_pr_finish` to sync the finalized SOR to that tracked review surface and stage it even when `--paths` would otherwise miss it
- kept the canonical `.adl` task-bundle SOR sync and `.adl/cards` compatibility mirror intact
- extended the Rust finish regressions to verify the tracked `docs/records` SOR publication path
- updated the shell finish harness to assert the tracked SOR path is staged and to seed an authored issue prompt fixture for newer lifecycle validation
- updated the finish skill contract and default workflow docs so they teach SOR finalization before PR open and tracked review-surface publication

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet
- Worktree-only paths remaining:
  - `adl/src/cli/pr_cmd.rs`
  - `adl/src/control_plane.rs`
  - `adl/src/cli/tests/pr_cmd_inline/finish.rs`
  - `adl/tools/card_paths.sh`
  - `adl/tools/test_pr_finish_default_stage_root.sh`
  - `adl/tools/skills/pr-finish/SKILL.md`
  - `adl/tools/skills/pr-finish/references/output-contract.md`
  - `docs/default_workflow.md`
  - `docs/records/README.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: bounded worktree update pending `pr finish`
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish -- --nocapture`
  - `bash adl/tools/test_pr_finish_default_stage_root.sh`
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish -- --nocapture` to verify the Rust finish path syncs the completed SOR to the tracked `docs/records` review surface without regressing existing finish behavior
  - `bash adl/tools/test_pr_finish_default_stage_root.sh` to verify the shell harness stages the tracked SOR review surface by default during finish
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` to confirm formatting remained clean after the control-plane and doc updates
- Results:
  - the focused Rust finish regressions passed
  - the shell finish harness passed
  - formatting check passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish -- --nocapture"
      - "bash adl/tools/test_pr_finish_default_stage_root.sh"
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
  determinism:
    status: PARTIAL
    replay_verified: unknown
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
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish -- --nocapture`
  - `bash adl/tools/test_pr_finish_default_stage_root.sh`
- Fixtures or scripts used:
  - Rust finish regression fixtures in `adl/src/cli/tests/pr_cmd_inline/finish.rs`
  - shell finish harness in `adl/tools/test_pr_finish_default_stage_root.sh`
- Replay verification (same inputs -> same artifacts/order):
  - the focused finish regressions repeatedly exercised the tracked SOR publication path under `docs/records/<scope>/tasks/issue-<padded>/sor.md`; local pre-finish branch publication has not been replay-verified yet
- Ordering guarantees (sorting / tie-break rules used):
  - the tracked publication path is deterministically derived from milestone scope and padded issue id rather than ad hoc output naming
- Artifact stability notes:
  - the tracked SOR path is stable across reruns for the same issue and no schema change was introduced to the SOR itself

## Security / Privacy Checks
- Secret leakage scan performed:
  - reviewed the changed code, docs, and tests; no secrets or tokens were introduced
- Prompt / tool argument redaction verified:
  - the change does not add prompt capture or tool-argument logging to the published review surface
- Absolute path leakage check:
  - recorded commands and paths in this SOR are repository-relative
- Sandbox / policy invariants preserved:
  - yes; the change stayed within finish/control-plane/docs/test surfaces

## Replay Artifacts
- Trace bundle path(s):
  - not applicable; this is a finish/publication workflow issue
- Run artifact root:
  - not applicable
- Replay command used for verification:
  - the focused Rust and shell finish regressions listed above
- Replay result:
  - passed; expected tracked-publication behavior was reproduced without manual intervention

## Artifact Verification
- Primary proof surface:
  - the tracked SOR publication path under `docs/records/<scope>/tasks/issue-<padded>/sor.md` as exercised by the focused finish regressions
- Required artifacts present:
  - yes
- Artifact schema/version checks:
  - the finish path still runs completed-phase SOR validation and the regressions verified that behavior while adding the tracked review surface
- Hash/byte-stability checks:
  - not separately run; path-level and completed-SOR validation behavior were the relevant proof standard here
- Missing/optional artifacts and rationale:
  - no full clippy or full-suite run was required for this bounded finish/publication change; the focused finish regressions and formatting check provided the intended proof surface

## Decisions / Deviations
- kept `.adl` task-bundle and `.adl/cards` output mirrors as local compatibility surfaces instead of turning this issue into a full public-record migration
- updated the older shell harness to seed an authored issue prompt fixture because newer lifecycle validation no longer accepts bootstrap-stub prompts in that test path
- kept this pre-finish SOR truthful to the current `worktree_only` state rather than claiming `pr_open` before publication actually happens

## Follow-ups / Deferred work
- after merge, `pr-closeout` should finalize the local closeout state and prune the 1449 worktree
