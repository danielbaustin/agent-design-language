# CLI Test Ownership Index And Helper Consolidation Candidates (#3624)

Issue: #3624
Status: implemented as index and safe-refactor routing

## Purpose

This index maps the CLI test forest to the owner binaries and compatibility
surfaces introduced during the v0.91.5 CLI refactor mini-sprint.

The goal is faster, more trustworthy proof selection: changes to
`adl-csdlc`, `adl-runtime`, `adl-review`, wrappers, or compatibility shims
should not reflexively trigger broad local test cycles when a focused owner
lane proves the touched behavior.

No tests are deleted, moved, or weakened by #3624.

## Source Inputs

- `adl/src/cli/tests/`
- `adl/src/cli/tests/pr_cmd_inline/`
- `adl/src/cli/tests/run_state/`
- `adl/tools/run_owner_validation_lane.sh`
- `docs/milestones/v0.91.5/MODULE_NAVIGABILITY_REVIEW_3612.md`
- `#3596`
- `#3598`
- `#3599`
- `#3612`

## Test Forest Snapshot

| Surface | Count | Notes |
| --- | ---: | --- |
| `adl/src/cli/tests/**/*.rs` | 48 Rust test files | CLI test forest across internal commands, PR control plane, runtime state, artifacts, and usage. |
| `adl/src/cli/tests/pr_cmd_inline/**/*.rs` | 25 Rust test files | Highest-volume PR lifecycle and wrapper-adjacent tests. |
| `adl/src/cli/tests/run_state/**/*.rs` | 5 Rust test files | Runtime run-state persistence/control tests. |
| owner-lane shell tests | 5 scripts | Fast command-family compatibility gates used by `run_owner_validation_lane.sh`. |

## Owner Validation Lanes

`adl/tools/run_owner_validation_lane.sh` is the current owner-lane validation
spine.

| Lane | Command | What it proves | Primary scripts |
| --- | --- | --- | --- |
| C-SDLC | `bash adl/tools/run_owner_validation_lane.sh csdlc` | Live command guidance, wrapper migration contract, run ambiguity policy, and control-plane observability contract. | `test_cli_owner_command_guidance.sh`, `test_cli_wrapper_migration_contract.sh`, `test_pr_run_ambiguity_policy.sh`, `test_control_plane_observability.sh` |
| Runtime | `bash adl/tools/run_owner_validation_lane.sh runtime` | `adl-runtime` compatibility boundary and fail-closed handoff for issue-mode inputs. | `test_adl_runtime_compatibility.sh` |
| Review | `bash adl/tools/run_owner_validation_lane.sh review` | `adl-review` compatibility boundary and review contract routing. | `test_adl_review_compatibility.sh` |
| All owner lanes | `bash adl/tools/run_owner_validation_lane.sh all --build` | All three owner boundaries after one shared build. | All owner-lane scripts with prebuilt binary overrides. |

## CLI Test Ownership Map

| Test surface | Owner family | Proof role | Use when touching |
| --- | --- | --- | --- |
| `pr_cmd_inline/basics.rs` | C-SDLC / wrapper | Issue create/init basics, source prompt validation, metadata guardrails, bootstrap behavior. | `pr.sh`, `adl pr`, issue body validation, card bootstrap. |
| `pr_cmd_inline/versioned_bootstrap.rs` | C-SDLC / prompt templates | Versioned prompt-template registry and card generation compatibility. | `docs/templates/prompts/`, renderer/schema paths, card lifecycle generation. |
| `pr_cmd_inline/lifecycle/start_ready.rs` | C-SDLC / wrapper | Design-time readiness, worktree binding, open-wave blocking, start/run lifecycle. | `pr run`, `doctor`, branch/worktree binding. |
| `pr_cmd_inline/lifecycle/diagnosis.rs` | C-SDLC / doctor | Doctor/readiness diagnostics and partial bundle behavior. | `doctor`, readiness JSON, lifecycle repair guidance. |
| `pr_cmd_inline/lifecycle/closeout.rs` | C-SDLC / closeout | Post-merge/no-PR closeout truth and worktree pruning. | `closeout`, post-merge cards, closed issue records. |
| `pr_cmd_inline/finish/**` | C-SDLC / finish-publication | Finish guardrails, path staging, output truth, PR body/linkage, janitor, local-vs-CI lane policy. | `finish`, PR publication, output card truth, validation lane selection. |
| `pr_cmd_inline/repo_helpers/**` | C-SDLC / GitHub integration helpers | Repo inference, issue metadata parity, PR linkage, prompt generation support. | GitHub metadata, origin parsing, issue/PR body repair. |
| `pr_cmd_inline/support.rs` | C-SDLC shared test support | Temp dirs, env lock, bootstrap support files, prompt-template fixture copy, authored card fixtures. | Test helper changes used by PR lifecycle tests. |
| `run_state/**` | Runtime | Run-state persistence, runtime control, failure taxonomy. | `adl-runtime run/resume`, run artifacts, runtime state persistence. |
| `internal_commands/**` | Runtime | Internal command routing for demo, instrumentation, learn, signing, control path, runtime state. | `adl-runtime demo/instrument/learn/keygen/sign/verify/artifact` command families. |
| `artifact_builders/**` | Runtime / artifacts | Artifact model builders, learning runtime summaries, cognitive/agency execution outputs. | Runtime artifact builders and schema-like output models. |
| `godel.rs` | Runtime / Gödel | Gödel CLI routing and artifact behavior. | `adl-runtime godel ...` and future Gödel mechanics activation. |
| `open_usage.rs` | C-SDLC / review / docs | CLI usage examples and command output guidance. | Help text, usage docs, command examples. |
| owner-lane shell scripts | Owner boundary smoke tests | Fast compatibility proof for owner binaries and wrapper guidance. | `adl-csdlc`, `adl-runtime`, `adl-review`, shim policy changes. |

## Helper Duplication Inventory

These are safe-consolidation candidates, not changes made by #3624.

| Candidate | Evidence | Risk | Required characterization before extraction |
| --- | --- | --- | --- |
| Bare git remote fixture setup | Repeated `git init bare`, origin setup, branch/worktree fixture setup across lifecycle and finish tests. | High: failure messages and repository topology are part of guardrail behavior. | Pre/post targeted tests for `start_ready`, `diagnosis`, `closeout`, and `finish/publication/flow`. |
| Root/worktree STP/SOR seeding | Repeated `fs::copy(issue_ref.issue_prompt_path(...), stp)` and `write_completed_sor_fixture(...)` across finish guardrails. | Medium-high: finish behavior depends on exact stale/local-only truth. | Pre/post tests for output truth, canonical surfaces, foreign bundle, sync/prompt, and publication flow. |
| Bootstrap support file copy | `copy_bootstrap_support_files` and `copy_versioned_prompt_templates` are already centralized in `support.rs`, but call sites are numerous. | Medium: helper is central; changing it fans out quickly. | Add assertions around copied file set before changing helper behavior. |
| GH fixture scripts | Several tests write inline `gh` shell fixtures for issue/PR metadata behavior. | Medium: inline scripts are verbose but make test behavior explicit. | Extract only after snapshotting stdout/stderr/error-class behavior for metadata/linkage tests. |
| Owner-lane shell binary overrides | Runtime/review compatibility scripts and `run_owner_validation_lane.sh --build` share override patterns. | Low-medium: scripts are small, but env var names are command contract. | Pre/post `run_owner_validation_lane.sh all --print-plan` and targeted compatibility scripts. |

## Validation Guidance

| Change type | Minimum local proof | When to broaden |
| --- | --- | --- |
| Live command guidance, prompt-template command strings, wrapper policy | `bash adl/tools/run_owner_validation_lane.sh csdlc` | Broaden only if generated cards or `pr.sh` execution behavior changes. |
| `adl-csdlc` prompt-template behavior | C-SDLC owner lane plus prompt-template renderer/schema command touched by the issue. | Broaden to targeted Rust tests if renderer internals change. |
| `adl-runtime` command routing | `bash adl/tools/run_owner_validation_lane.sh runtime` | Broaden to runtime/state tests if run/resume/artifact behavior changes. |
| `adl-review` command routing | `bash adl/tools/run_owner_validation_lane.sh review` | Broaden if review packet schemas or output contracts change. |
| `pr finish` output truth or PR body behavior | Targeted `pr_cmd_inline/finish/**` Rust tests plus C-SDLC owner lane. | Broaden to all PR lifecycle tests if shared helpers change. |
| Test helper extraction | Pre/post targeted tests for every call-site family touched. | Broaden to full CLI tests if shared support functions change behavior. |
| Docs-only index/policy changes | Owner lane relevant to the policy plus `git diff --check`. | Broad Rust tests not required unless executable behavior changes. |

## Deferred Cleanup Routes

| Deferred slice | Route | Required proof |
| --- | --- | --- |
| PR lifecycle fixture builder consolidation | Future C-SDLC test-helper refactor issue after #3582/#3622 stabilize prompt-card rendering. | Pre/post targeted PR lifecycle tests and exact failure-message preservation. |
| GH fixture script helper | Future GitHub metadata/linkage test cleanup issue. | Metadata/linkage tests before and after extraction. |
| Owner-lane shell helper extraction | Future validation-lane ergonomics issue if script duplication grows. | `--print-plan` parity and compatibility-script pass. |
| Runtime run-state helper cleanup | Runtime logging/action-log issue or later runtime test refactor. | Run-state persistence/control tests before and after. |

## Current Recommendation

Do not perform helper extraction in #3624.

The current shared helper surface already exists in `pr_cmd_inline/support.rs`;
the safer immediate win is the ownership index and validation guidance. Actual
helper consolidation should happen only when a code issue can run the targeted
pre/post tests and preserve failure messages.

## Validation

Focused validation for this issue:

- `find adl/src/cli/tests -type f -name '*.rs' | sort`
- `find adl/src/cli/tests -type f -name '*.rs' | wc -l`
- `find adl/src/cli/tests/pr_cmd_inline -type f -name '*.rs' | wc -l`
- `find adl/src/cli/tests/run_state -type f -name '*.rs' | wc -l`
- `bash adl/tools/run_owner_validation_lane.sh all --print-plan`
- `bash adl/tools/run_owner_validation_lane.sh csdlc`
- `git diff --check`

## Non-Claims

- This issue does not delete or rewrite tests.
- This issue does not reduce coverage.
- This issue does not move command behavior.
- This issue does not claim the full CLI test suite was run.
- This issue does not complete future helper consolidation.
