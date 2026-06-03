# v0.91.5 Refactor Safety Baseline

Issue: #3593
Umbrella: #3592
Captured: 2026-06-03T16:25:37Z
Status: validated

## Purpose

This packet establishes the safety baseline for the v0.91.5 CLI/refactor
mini-sprint before any command ownership split moves behavior. The goal is to
make later issues cite a concrete proof surface instead of relying on session
memory or a broad "green enough" assumption.

## Scope

In scope:

- Identify trusted validation lanes for the first CLI ownership split.
- Verify the PVF false-green finding from v0.91.4 is not still present in the
  actual release-policy script.
- Record command-characterization requirements for later issues.
- Record any unavailable broad validation as explicit substitute proof.

Out of scope:

- Moving command implementations between binaries.
- Renaming public commands.
- Introducing compatibility shims.
- Changing runtime behavior.

## Trusted Lanes

| Lane | Command | Required before | Proof role | Status |
| --- | --- | --- | --- | --- |
| Git patch hygiene | `git diff --check` | Every child issue | Catches whitespace and patch-format regressions. | passed |
| PVF runner contract | `bash adl/tools/test_run_pvf_validation_lane.sh` | #3594 and later | Proves lane aggregation, skipped/deferred/reused/release-gate statuses, print-plan behavior, credential blocking, invalid manifests, and nonzero failed aggregates. | passed |
| PVF CI release policy | `bash adl/tools/test_pvf_ci_release_policy.sh` | #3594 and later | Proves docs/runtime/release path-policy behavior and release-mode exit handling. | passed |
| Prompt-template schema parity | `adl/target/debug/adl tooling prompt-template validate-schemas` | Card/template work in the mini-sprint | Proves tracked structure schemas match active prompt templates. | passed |
| Issue-card structure | `adl/target/debug/adl tooling prompt-template validate-structure --kind stp|srp|sor --input <card>` | This issue and later regenerated-card work | Proves rendered card structure still matches the active template schemas. | passed |
| Provider/runtime focused tests | `cargo test --manifest-path adl/Cargo.toml --test provider_tests -- --nocapture` | #3598 | Focused runtime/provider characterization before `adl-runtime` command ownership changes. | planned for runtime split |
| Full Rust suite | `cargo test --manifest-path adl/Cargo.toml --all-features` | Before large behavior-moving refactors | Broad regression net for later behavior-preserving movement. | not required for this docs/evidence baseline |
| Clippy all targets | `cargo clippy --manifest-path adl/Cargo.toml --all-features --all-targets -- -D warnings` | Before large behavior-moving refactors | Static Rust hygiene for later movement. | not required for this docs/evidence baseline |

## False-Green Disposition

The v0.91.4 external review finding `R1` warned that one PVF release-lane test
could exit under `set -e` before its assertions. The current tracked script is
`adl/tools/test_pvf_ci_release_policy.sh`; there is no
`adl/tools/test_run_pvf_validation_lane_release_policy.sh`.

Current script disposition:

- `docs_status` and `runtime_status` are captured under `set +e`.
- `release_status` is also captured under `set +e`.
- The script returns to `set -e` only after preserving the runner status.
- Python assertions and `grep` assertions run after each expected status is
  checked.

The cited release-lane false-green class is fixed in the current tracked
release-policy script and was proved by `bash
adl/tools/test_pvf_ci_release_policy.sh` during this issue.

## Command Characterization Requirements

Every later CLI split issue must characterize old and new command behavior
before claiming compatibility:

| Surface | Characterization required |
| --- | --- |
| Exit code | Record expected success/failure codes for old and new command forms. |
| Stdout/stderr | Capture meaningful stable lines, especially warnings, routing decisions, and failure messages. |
| Artifact writes | Record generated or modified paths, including whether they are tracked, ignored, or worktree-local. |
| Error taxonomy | Preserve stable error class or fail-closed behavior for ambiguous invocations. |
| Help output | Confirm the new binary advertises the correct owner surface without hiding the compatibility path. |
| Generated cards | Verify generated prompt cards do not fossilize deprecated command strings. |
| Wrapper path | Confirm `adl/tools/pr.sh` and `workflow-conductor` remain canonical until the wrapper migration issue changes that truth. |

## Handoff Gates

Later issues should not proceed past their implementation boundary unless these
issue-specific gates are true:

| Issue | Gate |
| --- | --- |
| #3594 | Command inventory cites this baseline and corrects stale validation selectors. |
| #3595 | Run ambiguity tests fail closed for unknown/ambiguous inputs. |
| #3596 | `adl-csdlc` compatibility is routing-only unless explicitly widened. |
| #3597 | `adl/tools/pr.sh`, `workflow-conductor`, generated cards, and templates have one documented migration truth. |
| #3598 | Runtime split runs focused provider/runtime characterization before publication. |
| #3599 | Review split proves review commands without changing review semantics. |
| #3600 | Mini-sprint review checks compatibility, generated-card policy, validation lane separation, and residual split risk. |

## Observability Note

During this issue, invoking the prompt-template CLI help path through a fallback
`cargo run` command did not return promptly. This issue does not change logging
or CLI observability, but the behavior is evidence for the planned follow-on
observability work: long-running doctor/help/validator commands need visible,
structured progress and enough diagnostic detail to explain where they are
waiting.

## Validation Log

| Command | Result | Notes |
| --- | --- | --- |
| `git diff --check` | PASS | Patch hygiene check passed after adding this packet. |
| `bash adl/tools/test_run_pvf_validation_lane.sh` | PASS | Required focused PVF runner proof passed. |
| `bash adl/tools/test_pvf_ci_release_policy.sh` | PASS | Required focused release-policy proof passed. |
| `adl/target/debug/adl tooling prompt-template validate-schemas` | PASS | Tracked structure schemas matched active templates. |
| `adl/target/debug/adl tooling prompt-template validate-structure --kind stp --input .adl/v0.91.5/tasks/issue-3593__v0-91-5-refactor-mini-sprint-1-8-establish-refactor-safety-baseline/stp.md` | PASS | Issue STP structure matched the active template schema. |
| `adl/target/debug/adl tooling prompt-template validate-structure --kind srp --input .adl/v0.91.5/tasks/issue-3593__v0-91-5-refactor-mini-sprint-1-8-establish-refactor-safety-baseline/srp.md` | PASS | Issue SRP structure matched the active template schema. |
| `adl/target/debug/adl tooling prompt-template validate-structure --kind sor --input .adl/v0.91.5/tasks/issue-3593__v0-91-5-refactor-mini-sprint-1-8-establish-refactor-safety-baseline/sor.md` | PASS | Issue SOR structure matched the active template schema. |

## Non-Claims

- This packet does not claim the CLI has already been decomposed.
- This packet does not claim the full Rust suite was run.
- This packet does not approve deeper module surgery, workspace crate splits, or
  OpenTelemetry implementation in this mini-sprint wave.
- This packet does not change `adl/tools/pr.sh` canonical status.
