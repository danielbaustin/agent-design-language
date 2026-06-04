# v0.91.5 Refactoring Mini-Sprint Code Review

Date: 2026-06-04

Review type: code-focused mini-sprint review

Reviewed scope:

- First CLI/refactor mini-sprint umbrella: `#3592`
- Child issues: `#3593`, `#3594`, `#3595`, `#3596`, `#3597`, `#3598`, `#3599`, `#3600`
- Direct refactor follow-ons reviewed in the same pass: `#3612`, `#3614`, `#3622`
- Merged PRs / commits reviewed:
  - `#3601` / `cee4659f` - refactor safety baseline
  - `#3602` / `f9543326` - CLI command inventory
  - `#3603` / `b226c0bc` - PR run ambiguity policy
  - `#3604` / `5637d958` - `adl-csdlc` compatibility binary
  - `#3605` / `23f621f1` - wrapper migration contract
  - `#3606` / `86f6091a` - `adl-runtime` compatibility binary
  - `#3608` / `21838c6e` - `adl-review` compatibility binary
  - `#3613` / `38deeb63` - mini-sprint review and follow-on routing
  - `#3626` / `0f951a72` - module navigability review helper
  - `#3627` / `1ba62efa` - deferred helper binary review
  - `#3645` / `e1dd930c` - prompt-template values split

## Executive Summary

The refactoring mini-sprint is technically sound. The owner-binary split creates
clearer command ownership without silently replacing the canonical `pr.sh`
workflow, and the prompt-template values extraction appears behavior-preserving.

The main review result is **no P1/P2 code blockers found** in the code surfaces
reviewed.

One concrete `P3` issue remains: the new `adl-runtime` help text does not list
`--allow-unsigned`, even though the runtime parser supports it and tests rely on
it for fixture execution.

## Findings

### P3: `adl-runtime run --help` omits the supported `--allow-unsigned` flag

Evidence:

- `adl-runtime` usage text lists runtime flags in `adl/src/cli/mod.rs`.
- The usage line includes `--print-plan`, `--print-prompts`, `--trace`, `--run`,
  `--resume`, `--steer`, `--overlay`, `--out`, `--quiet`, and `--open`, but not
  `--allow-unsigned`.
- The runtime parser supports `--allow-unsigned` in `adl/src/cli/run.rs`.
- Runtime smoke coverage uses unsigned execution for fixture runs.

Why it matters:

This is not a behavior regression, but it weakens the new owner-binary UX at the
moment the refactor is trying to make command ownership clearer. Operators using
`adl-runtime run --help` will not see a flag they may need for development or
fixture workflows.

Suggested fix:

Add `--allow-unsigned` to the `adl-runtime run` usage string and keep the runtime
compatibility tests aligned with that help surface.

## Code Surfaces Reviewed

Primary executable surfaces:

- `adl/src/bin/adl_csdlc.rs`
- `adl/src/bin/adl_runtime.rs`
- `adl/src/bin/adl_review.rs`
- `adl/src/cli/mod.rs`
- `adl/tools/pr.sh`
- `adl/tools/run_owner_validation_lane.sh`
- `adl/tools/test_pr_run_ambiguity_policy.sh`
- `adl/tools/test_cli_wrapper_migration_contract.sh`
- `adl/tools/test_cli_owner_command_guidance.sh`
- `adl/tools/test_adl_runtime_compatibility.sh`
- `adl/tools/test_adl_review_compatibility.sh`
- `adl/src/csdlc_prompt_editor.rs`
- `adl/src/csdlc_prompt_editor/values.rs`
- `adl/src/cli/tooling_cmd/prompt_template.rs`
- `adl/src/cli/tooling_cmd/tests/prompt_template.rs`
- `adl/tests/cli_smoke.rs`

Supporting review/evidence documents:

- `docs/milestones/v0.91.5/REFACTOR_SAFETY_BASELINE_3593.md`
- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md`
- `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md`
- `docs/milestones/v0.91.5/CLI_RUNTIME_COMPATIBILITY_3598.md`
- `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md`
- `docs/milestones/v0.91.5/CLI_REFACTOR_MINI_SPRINT_REVIEW_3600.md`
- `docs/milestones/v0.91.5/MODULE_NAVIGABILITY_REVIEW_3612.md`
- `docs/milestones/v0.91.5/HELPER_BINARY_CANDIDATE_REVIEW_3614.md`
- `docs/milestones/v0.91.5/CSDLC_PROMPT_EDITOR_SPLIT_3622.md`

## What Looks Good

### Owner-binary split is coherent

`adl-csdlc`, `adl-runtime`, and `adl-review` now express a clear three-owner
shape:

- `adl-csdlc` owns C-SDLC/tooling compatibility and rejects runtime execution.
- `adl-runtime` owns workflow/runtime/provider/demo/helper surfaces and rejects
  C-SDLC issue work.
- `adl-review` owns review tooling and rejects runtime/C-SDLC command families.

The split preserves the current migration truth that `adl/tools/pr.sh run
<issue>` remains the canonical agent-facing issue binder.

### PR run ambiguity policy is meaningfully safer

The shell wrapper now separates issue-mode and runtime-mode operands more
carefully:

- numeric operands remain issue-mode
- ADL YAML operands remain runtime-mode
- issue flags on runtime operands fail closed
- runtime flags on issue operands fail closed
- generated-card scans prevent stale runtime-through-PR command strings from
  reappearing in prompt templates

This directly addresses the command-family ambiguity that motivated the sprint.

### Prompt-template values split appears behavior-preserving

The extraction into `adl/src/csdlc_prompt_editor/values.rs` keeps a narrow
responsibility boundary:

- values YAML loading
- locked-system/editable-values separation
- sample values generation
- deterministic YAML emission helpers

The reviewed tests cover:

- all five prompt card kinds
- locked vs editable field enforcement
- invalid enum rejection
- rendered-card round trip
- locked prose drift failure
- multiline editable values

## Validation Performed

Commands run during this review:

```bash
bash adl/tools/run_owner_validation_lane.sh all --build
```

Result: passed.

This covered:

- C-SDLC owner command guidance
- wrapper migration contract
- PR run ambiguity policy
- C-SDLC control-plane observability contract
- runtime compatibility boundary
- review compatibility boundary

```bash
cargo test --manifest-path adl/Cargo.toml csdlc_prompt_editor::tests -- --nocapture
```

Result: passed.

This covered eight prompt editor tests, including the values split and
render/import round-trip behavior.

## Residual Risk

- This was a focused code review, not a full repository review.
- I did not rerun the full Rust test suite.
- I did not run full GitHub CI for this review packet.
- I did not inspect every line of all downstream refactor follow-on issues,
  only the code surfaces most directly changed by the mini-sprint and the prompt
  editor split.
- The review did not cover the later octocrab mini-sprint; that should receive a
  separate code-focused review packet.

## Review Conclusion

The refactoring mini-sprint is in good shape from a code-review perspective.
The owner boundaries are materially clearer, the compatibility tests are
targeted, and the prompt-template extraction is narrow enough to be reviewable.

Recommended follow-up:

- Create or route one small docs/tools issue to add `--allow-unsigned` to
  `adl-runtime run --help`.
- Use this same code-first review pattern for the octocrab mini-sprint.
