# v0.90 Maintainability Hotspot Split Plan

## Purpose

This plan carries WP-16 finding F8 from `v0.89.1` into explicit `v0.90`
maintainability work. It is intentionally a next-milestone plan, not a
retroactive `v0.89.1` release blocker.

Source finding:
`docs/milestones/v0.89.1/INTERNAL_REVIEW_v0.89.1.md` F8,
"Large module hotspots remain residual maintainability debt."

Inventory command:

```bash
bash adl/tools/report_large_rust_modules.sh
```

Inventory snapshot captured for this plan:

```text
Rust module size watch report
scan roots: adl/src, adl/tests
thresholds: watch>=800, review>=1000, rationale>=1500

Path                                                                      LoC  Level
-----------------------------------------------------------------------  ----  ---------
adl/src/cli/identity_cmd/tests.rs                                        1744  RATIONALE
adl/src/cli/run_artifacts/runtime/trace_validation.rs                    1368  REVIEW
adl/src/cli/tooling_cmd/tests.rs                                         1361  REVIEW
adl/tests/provider_tests.rs                                              1358  REVIEW
adl/src/instrumentation.rs                                               1353  REVIEW
adl/src/cli/run_artifacts/cognitive/state_artifacts.rs                   1319  REVIEW
adl/src/demo/v086_review_surface.rs                                      1207  REVIEW
adl/src/cli/tests/run_state/persistence.rs                               1184  REVIEW
adl/tests/adl_tests.rs                                                   1184  REVIEW
adl/src/cli/tests/artifact_builders/learning_runtime/artifact_models.rs  1176  REVIEW
adl/src/adl/tests.rs                                                     1157  REVIEW
adl/src/cli/tests/internal_commands.rs                                   1133  REVIEW
adl/src/execute/tests.rs                                                 1122  REVIEW
adl/src/cli/tests/pr_cmd_inline/basics.rs                                1071  REVIEW
adl/tests/execute_tests/delegation_resume/pause_resume.rs                1028  REVIEW
adl/src/trace.rs                                                         1024  REVIEW
adl/src/cli/tests/pr_cmd_inline/repo_helpers.rs                          1023  REVIEW
adl/src/remote_exec.rs                                                   1014  REVIEW
adl/src/execute/runner.rs                                                1011  REVIEW
adl/src/cli/run_artifacts_types.rs                                       1004  REVIEW
adl/src/cli/pr_cmd_cards.rs                                              1001  REVIEW
```

## Split Policy

These hotspots are not automatically failing gates. A v0.90 split issue should
be opened when one of the following is true:

- the module is still at `RATIONALE` level
- the module is a high-churn test surface that slows review
- the module mixes unrelated responsibilities that can be split without
  changing public behavior
- a planned v0.90 feature would otherwise expand the file further

Each split PR must preserve runtime semantics. Test-only splits should preserve
test names or use clear module names so `cargo test` filters remain easy to
translate.

## Proposed Child Issue Wave

| Order | Proposed Issue | Target Surface | Split Shape | Validation |
|---|---|---|---|---|
| 1 | `[v0.90][maintainability] Split identity command tests by subcommand family` | `adl/src/cli/identity_cmd/tests.rs` | Move contract, argument-validation, identity-profile, and command-dispatch tests into focused child modules under `identity_cmd/tests/`. | `cargo test cli::identity_cmd::tests --lib -- --nocapture`; `cargo fmt -- --check` |
| 2 | `[v0.90][maintainability] Split runtime trace validation by artifact family` | `adl/src/cli/run_artifacts/runtime/trace_validation.rs` | Extract schema/envelope checks, required-artifact checks, and error-reporting helpers into sibling modules. | `cargo test run_artifacts runtime trace --lib -- --nocapture`; `cargo fmt -- --check` |
| 3 | `[v0.90][maintainability] Split tooling command tests by command group` | `adl/src/cli/tooling_cmd/tests.rs` | Move card-prompt, review-contract, structured-prompt, and wave-generation tests into group-specific modules. | `cargo test tooling_cmd --lib -- --nocapture`; `cargo fmt -- --check` |
| 4 | `[v0.90][maintainability] Split provider integration tests by provider family` | `adl/tests/provider_tests.rs` | Split HTTP, profile expansion, native provider, local/mock provider, and CLI-provider tests under `adl/tests/provider_tests/`. | `cargo test --test provider_tests -- --nocapture`; `cargo fmt -- --check` |
| 5 | `[v0.90][maintainability] Split instrumentation helpers by concern` | `adl/src/instrumentation.rs` | Extract trace loading, normalization, diffing, and graph export into focused instrumentation modules. | `cargo test instrumentation --lib -- --nocapture`; `cargo test --test instrument_tests -- --nocapture`; `cargo fmt -- --check` |

## Execution Notes

- Do not combine these child issues into one broad refactor PR.
- Prefer one module family per PR, with a small facade where callers currently
  import the parent module.
- If a split reveals behavior changes, stop and create a separate bug issue
  instead of hiding behavior changes inside a maintainability PR.
- If a target has already been reduced below review threshold by another PR,
  close or retitle the child issue rather than forcing a needless split.

## Current Disposition

This issue resolves the WP-16 F8 planning gap by making the v0.90 follow-on
wave explicit. The actual child splits should execute as separate v0.90 issues
so each one can stay reviewable and merge under green CI.
