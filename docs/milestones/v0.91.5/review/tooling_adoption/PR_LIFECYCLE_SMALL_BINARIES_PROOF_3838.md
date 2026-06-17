# PR Lifecycle Small Binaries Proof (`#3838`)

## Summary

`#3838` carries the small-binary split beyond validator/editor surfaces and
adds direct lifecycle binaries for the remaining C-SDLC workflow commands that
were still entering Rust through the broad:

```text
adl pr ...
```

compatibility path.

The canonical operator-facing workflow spine does **not** change:

- `adl/tools/pr.sh` remains the public issue-work wrapper
- `workflow-conductor` still routes lifecycle work through `adl/tools/pr.sh`

What changes is the implementation owner beneath that wrapper.

## Direct Lifecycle Binaries Added

- `adl-pr-create`
- `adl-pr-init`
- `adl-pr-repair-issue-body`
- `adl-pr-run`
- `adl-pr-doctor`
- `adl-pr-ready`
- `adl-pr-preflight`
- `adl-pr-finish`
- `adl-pr-validation`
- `adl-pr-closeout`

Each binary dispatches one bounded lifecycle subcommand directly instead of
routing through the broad `adl` umbrella binary first.

## Wrapper Delegation Truth

`adl/tools/pr.sh` now delegates lifecycle commands in this order:

1. preserve the legacy broad override via `ADL_PR_RUST_BIN`
2. honor an explicit per-command direct override such as:
   - `ADL_PR_DOCTOR_BIN`
   - `ADL_PR_RUN_BIN`
   - `ADL_PR_FINISH_BIN`
   - `ADL_PR_CLOSEOUT_BIN`
3. prefer a fresh built direct binary under `adl/target/debug/`
4. fall back to `cargo run --bin <direct-binary> -- ...`
5. use the broad `adl` compatibility path only when no direct binary mapping
   exists

This preserves fail-closed compatibility while moving the warm-path owner to
small direct binaries.

## Scope Covered

This issue now gives direct binary ownership to the remaining workflow lanes
named in the toolkit-simplification sprint acceptance:

- issue bootstrap and issue-body repair
- readiness / doctor / preflight classification
- execution binding (`pr run` issue mode)
- publication / validation
- closeout

It does **not** make `adl/tools/pr.sh` non-canonical, and it does **not**
claim `workflow-conductor` or generated cards should stop teaching the wrapper.

The focused wrapper-delegation proof now exercises direct dispatch for:

- `doctor`
- issue-mode `run`
- `finish`
- `validation`
- `closeout`

It also continues to prove that the broad `ADL_PR_RUST_BIN` compatibility
override retains precedence over per-command direct binary overrides.

## Validation

Focused proofs that passed:

- `cargo build --manifest-path adl/Cargo.toml --bin adl-pr-create --bin adl-pr-init --bin adl-pr-repair-issue-body --bin adl-pr-run --bin adl-pr-doctor --bin adl-pr-ready --bin adl-pr-preflight --bin adl-pr-finish --bin adl-pr-validation --bin adl-pr-closeout`
- `bash adl/tools/test_pr_small_binary_delegation.sh`
- `bash adl/tools/test_pr_doctor_prefers_built_binary.sh`
- `bash adl/tools/test_pr_ready_prefers_built_binary.sh`
- `bash adl/tools/test_control_plane_observability.sh`
- `bash adl/tools/run_owner_validation_lane.sh csdlc`
- `git diff --check`

## Non-Claims

- This issue does not retire `adl/tools/pr.sh`.
- This issue does not claim `adl-csdlc issue ...` is the public command taught
  to operators.
- This issue does not split `card`, `output`, or `cards` shell-only helper
  paths into direct binaries.
- This issue does not delete the broad `adl pr ...` compatibility route.
