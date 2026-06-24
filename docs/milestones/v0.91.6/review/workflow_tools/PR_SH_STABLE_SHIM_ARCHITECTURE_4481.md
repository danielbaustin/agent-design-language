# pr.sh Stable Shim Architecture

Issue: `#4481`
Status: architecture packet with first implementation slice
Milestone: `v0.91.6`
Audience: ADL operators and workflow-tooling implementers

## Summary

`adl/tools/pr.sh` should remain as the familiar operator entrypoint, but it
should stop being the place where high-change C-SDLC workflow policy lives.
The target architecture is a small, stable compatibility shim over typed ADL
commands and narrow `adl-pr-*` binaries.

The shim should own shell compatibility, user-facing help, environment setup,
and dispatch. Typed Rust/ADL command modules should own lifecycle policy,
GitHub transport, worktree binding, card readiness, validation selection, PR
state classification, closeout pruning, and machine-readable output contracts.

This is not a rewrite plan for one PR. It is a migration boundary and routing
map for small implementation slices. This issue also lands the first low-risk
slice: `pr.sh` now sources separate delegation and usage helpers, with
observable fail-closed loading for those helpers.

## Source Evidence

Tracked source and policy inputs used for this packet:

- `adl/tools/pr.sh`
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd/`
- `adl/src/bin/adl_pr_*.rs`
- `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`
- `docs/architecture/VALIDATION_LANE_SELECTOR.md`
- `.adl/v0.91.6/sprints/issue-4417__v0-91-6-tools-mini-sprint-validation-throughput-and-lifecycle-automation/SPRINT_ACTIVITY_LOG.md`
- `docs/milestones/v0.91.7/PLANNING_SOURCE_CAPTURE_v0.91.7.md`
- `docs/milestones/v0.91.7/WBS_v0.91.7.md`

Additional live workflow evidence from the just-completed `#4484` mini-sprint:

- `#4487` / PR `#4495`: worktree-safe lifecycle truth and bootstrap-stub
  pre-bind blockers.
- `#4488` / PR `#4496`: GitHub projection validation, direct PR watch, and
  green-draft classification.
- `#4489` / PR `#4497`: focused finish validation profile selection for
  GitHub projection/watch paths.

The previously mentioned `PR_SH_FAILURE_REGISTER_4481.md` and
`PR_SH_RECENT_FAILURE_CAPTURE_2026-06-24.md` files are not present on current
`main`; this packet therefore cites tracked retained evidence that exists on
the branch.

## Current State Inventory

`adl/tools/pr.sh` started this issue as a mixed shell surface of roughly 3,100 lines. It
contains four different categories of behavior:

| Category | Current examples | Target owner |
| --- | --- | --- |
| Compatibility entrypoint | command aliases, usage text, old option shapes | `pr.sh` shim |
| Environment and delegate selection | direct `adl-pr-*` binary preference, cargo fallback, build-lock liveness, token environment wiring | `pr.sh` shim plus typed delegate policy |
| Workflow policy | issue bootstrap, card readiness, open-wave preflight, worktree binding, finish guards, closeout rules | typed ADL command modules |
| Mutation and proof | GitHub issue/PR mutation, validation watch, SOR fact emission, closeout pruning | typed ADL command modules |

Typed command surfaces already exist:

- `adl pr create`
- `adl pr init`
- `adl pr repair-issue-body`
- `adl pr run`
- `adl pr doctor`
- `adl pr finish`
- `adl pr validation`
- `adl pr watch`
- `adl pr closing-linkage`
- `adl pr issue`
- `adl pr closeout`

Small binary entrypoints also exist for most lifecycle commands:

- `adl-pr-create`
- `adl-pr-init`
- `adl-pr-repair-issue-body`
- `adl-pr-run`
- `adl-pr-doctor`
- `adl-pr-finish`
- `adl-pr-validation`
- `adl-pr-closeout`
- `adl-pr-closing-linkage`
- `adl-pr-preflight`
- `adl-pr-ready`

Recent work has already moved several high-risk policies into typed modules:

- `doctor/preflight.rs` owns queue preflight, session-ledger claim checking,
  and card-run-readiness classification.
- `doctor/card_lifecycle.rs` owns lifecycle card readiness classification.
- `finish_support.rs` owns finish validation profile selection, stale-base
  guards, SOR fact emission, PR update/ready/merge flow, and closeout
  handoff.
- `github.rs` and `github/transport.rs` own typed GitHub projection,
  validation reports, issue/PR mutation, retry/timeout behavior, and
  observability events.
- `lifecycle/*` owns closeout reconciliation and cleanup surfaces.

## Implemented Slice In This Issue

This issue makes `pr.sh` smaller without changing its public command contract:

| File | Responsibility after this slice |
| --- | --- |
| `adl/tools/pr.sh` | Stable compatibility entrypoint, legacy shell commands not yet migrated, sourced helper loading, and main dispatch |
| `adl/tools/pr_delegate.sh` | Rust/typed delegate discovery, freshness checks, direct small-binary selection, cargo fallback liveness, and delegate execution |
| `adl/tools/pr_usage.sh` | Usage text and compatibility command help |

The split removes the delegate and usage sections from `pr.sh` while preserving
the existing command entrypoint. `pr.sh` now fails closed if a required helper
is missing and emits a redacted observability event before exiting:

- `stage=source-helper result=ok helper=...`
- `stage=source-helper result=missing helper=...`

The shell wrapper inventory was refreshed so these helper files and current
validation-manager/proof-lane scripts are tracked by the existing inventory
test. Copy-based shell fixtures now copy the helper family whenever they copy
`pr.sh`, which makes the new dependency explicit in tests.

The validation lane selector was also updated so the new helpers map to the
existing C-SDLC owner lane and the prompt-template shell test maps to the
existing prompt-template contract lane. This keeps `pr finish` fail-closed
without treating the new helper names as unmapped surfaces.

Publication exposed a narrow finish-runner parity gap: the selector can emit a
combined CI-policy command while the runner only knew the component commands.
This issue fixes that specific gap by decomposing the combined command into
the already-supported component checks.

The rebased publication path exposed a second runner parity gap for the
prompt-template workflow integration command. That command was already a
declared focused validation surface, but `pr finish` did not yet execute it
directly. This issue also adds that exact runner mapping and regression
coverage so the tailored prompt-template lane can complete through the normal
finish path.

## Failure Modes To Design Out

The target design should remove these recurring failure classes from shell
policy:

| Failure family | Evidence | Architectural response |
| --- | --- | --- |
| Root checkout drift and tracked work on `main` | Session coordination policy and repeated operator corrections during v0.91.5/v0.91.6 | make worktree binding, root-main guards, and branch/worktree ownership typed invariants |
| Open PR wave and closing-issue scan latency | `#4417` activity log entries for `pr.list.wave`, `pr.view.closing_issues`, and `#4479` routing | typed preflight must prefer canonical local identity and bound queue policy over repo-scale scans |
| Bootstrap card/source residue discovered too late | `#4417` card sweep, `#4487` | typed doctor/run pre-bind gates must fail before execution or publication |
| Delegate build-lock contention | `#4417` activity log and `#4486` route | shell shim may select binaries, but liveness classification and guidance must be typed and machine-readable |
| Draft/check state conflation | `#4488` | typed PR validation must expose projection state separately from raw check rollups |
| Over-broad finish validation | `#4417` finish-path observations, `#4489` | finish must consume deterministic path/profile metadata and run focused proof where sufficient |
| Raw GitHub fallback and manual provenance | provider review outputs and no-`gh` operating rule | typed GitHub transport is the only steady-state mutation/projection path |
| Partial closeout truth | `#4417` closeout-truth repair notes and `#4437` route | closeout must normalize STP/SIP/SRP/SOR truth in one typed terminal path |

## Target Architecture

### Layer 1: Stable Operator Shim

`adl/tools/pr.sh` should become a low-churn shell compatibility layer.

It should keep:

- public command names and common aliases
- concise usage/help text
- compatibility translation for old flags where still supported
- delegate binary discovery
- minimal token/environment handoff without printing secrets
- deterministic error messages when no typed delegate exists

It should not keep:

- lifecycle readiness policy
- GitHub API logic
- card lifecycle interpretation
- validation profile selection
- SOR fact construction
- closeout reconciliation
- worktree pruning policy
- repo-scale PR-wave scanning policy

### Layer 2: Typed Command Facade

`adl pr *` remains the canonical command namespace for C-SDLC lifecycle
operations. Each command should have:

- explicit argument parsing
- JSON-safe machine output where requested
- `adl_event` observability on the human/diagnostic channel
- fail-closed behavior before remote mutation
- focused tests for accepted and rejected states
- no hidden dependency on raw `gh`

### Layer 3: Small Binary Entry Points

`adl-pr-*` binaries provide fast, narrow dispatch for hot lifecycle commands.
The shim should prefer these binaries when fresh and executable. Cargo fallback
must stay diagnosable and bounded, not an invisible blocking path.

### Layer 4: Shared Typed Subsystems

High-change policy should live in shared modules with narrow contracts:

- `git_support`: branch, root, worktree, and dirty-state checks
- `doctor`: readiness, queue preflight, card lifecycle, and session-ledger
  policy
- `github`: issue/PR projection and mutation through typed transport
- `finish_support`: finish validation, PR body construction, SOR facts,
  ready/merge handling
- `lifecycle`: closeout reconciliation and pruning
- validation selector/profile code: path ownership and focused proof selection
- prompt-template/card tooling: card generation and structure validation

## Migration Map

| `pr.sh` command | Current typed target | Migration status | Next boundary |
| --- | --- | --- | --- |
| `create` | `adl pr create`, `adl-pr-create` | typed mutation exists | keep shell as compatibility only |
| `init` | `adl pr init`, `adl-pr-init` | typed bootstrap exists | finish removing shell-owned card/body policy |
| `repair-issue-body` | `adl pr repair-issue-body`, `adl-pr-repair-issue-body` | typed mutation exists | keep source-prompt validation in Rust |
| `run` / `start` | `adl pr run`, `adl-pr-run` | typed bind exists | move any remaining shell worktree/bootstrap logic out |
| `doctor` | `adl pr doctor`, `adl-pr-doctor` | typed readiness exists | make all preflight blockers structured and bounded |
| `ready` | `doctor --mode ready` | deprecated compatibility alias | keep alias in shim only |
| `preflight` | `doctor --mode preflight` | deprecated compatibility alias | keep alias in shim only |
| `finish` | `adl pr finish`, `adl-pr-finish` | typed publication exists | finish decomposing validation/SOR/PR body helpers |
| `validation` | `adl pr validation`, `adl-pr-validation` | typed GitHub check projection exists | keep shell as compatibility only |
| `watch` | `adl pr watch` | typed issue watcher exists | add dedicated `adl-pr-watch` if hot-path latency requires it |
| `closing-linkage` | `adl pr closing-linkage`, `adl-pr-closing-linkage` | typed CI guard exists | keep shell as compatibility only |
| `issue` | `adl pr issue` | typed issue list/view/create/edit/comment/close exists | add dedicated `adl-issue`/sub-binaries only where latency requires it |
| `closeout` | `adl pr closeout`, `adl-pr-closeout` | typed closeout exists | make terminal card truth and worktree pruning fully typed |
| `cards` | legacy shell card helper | partial legacy | retire or route through prompt-template/card tools |

## Compatibility Policy

1. `adl/tools/pr.sh` remains the supported operator entrypoint through
   `v0.91.7`.
2. Every supported `pr.sh` subcommand must delegate to `adl pr *` or an
   `adl-pr-*` binary before doing lifecycle work.
3. Deprecated aliases such as `ready` and `preflight` may remain only as
   argument translation into `doctor` modes.
4. Shell-only behavior is allowed only for:
   - usage text
   - delegate discovery
   - lock acquisition for cargo fallback
   - compatibility argument normalization
   - clear failure when a typed delegate is missing
5. Remote mutation must be typed. No steady-state path may fall back to raw
   `gh` or connector mutation.
6. Secrets must be passed through approved environment/token resolver paths and
   never printed.
7. Existing command shapes should fail with actionable migration guidance
   before they are removed.

## Failure Policy

The stable-shim control plane must preserve these fail-closed rules:

- fail before remote mutation when local identity, cards, worktree, branch, or
  PR base is ambiguous
- fail before execution bind when source prompts or design cards are bootstrap
  stubs
- fail before publication when the branch is stale against `origin/main`
- fail before merge when checks are pending, failed, cancelled, or timed out
- treat green-but-draft as a distinct lifecycle state, not failed CI
- block tracked implementation on root `main`
- block blind writes when the session ledger shows active ownership by another
  session
- do not hide manual fallback; record it in issue or sprint truth

## Validation And Observability Strategy

Validation should be proportional to the changed surface:

- shim-only changes: shell contract tests, delegate-resolution tests,
  `git diff --check`
- typed command changes: focused Rust unit tests for the affected command and
  JSON contract
- GitHub projection changes: mocked typed-transport tests plus one live
  repo-native validation only when explicitly required
- finish/validation changes: path-profile selector tests plus focused
  publication/validation facts tests
- card/template changes: prompt-template values, render, structure, and schema
  validation
- release-gate changes: release-gate proof rather than local focused proof
  alone

Machine-readable outputs should stay parse-safe. Human-facing observability
should remain `adl_event` records with redaction and path hygiene. Logging and
observability behavior must preserve the current contract: machine payloads on
stdout, human-oriented events on stderr by default, with explicit compatibility
paths only when documented.

Validation run for this implementation slice:

- `bash -n adl/tools/pr.sh`
- `bash -n adl/tools/pr_delegate.sh`
- `bash -n adl/tools/pr_usage.sh`
- `bash adl/tools/test_pr_small_binary_delegation.sh`
- `bash adl/tools/test_pr_delegate_cargo_fallback_liveness.sh`
- `bash adl/tools/test_pr_delegate_prefers_primary_checkout_binary.sh`
- `bash adl/tools/test_pr_doctor_prefers_built_binary.sh`
- `bash adl/tools/test_pr_ready_prefers_built_binary.sh`
- `bash adl/tools/test_pr_help_and_card_open.sh`
- `bash adl/tools/test_pr_run_ambiguity_policy.sh`
- `bash adl/tools/test_shell_wrapper_inventory.sh`
- `bash adl/tools/test_select_validation_lanes.sh`
- `bash adl/tools/test_validation_manager.sh`
- `cargo test --manifest-path adl/Cargo.toml --bin adl-pr-finish finish_runner_executes_combined_ci_policy_selector_command`
- `git diff --check`

## Proposed Follow-On Work

The implementation should be split into small slices. Some are already done or
partially done by recent issues; they should be consumed rather than duplicated.

| Slice | Routing | Notes |
| --- | --- | --- |
| Bound doctor/run PR-wave scans | done through `#4485` / related fixes | consume as current baseline |
| Delegate build-lock liveness | done/folded through `#4486` | verify with future shim tests |
| Worktree-safe lifecycle truth and bootstrap blockers | done through `#4487` | consume as current baseline |
| GitHub projection validation and PR metadata janitor | done through `#4488` | consume as current baseline |
| Focused finish validation profiles | done through `#4489` | consume as current baseline |
| Move remaining `cards` shell helper to prompt-template/card tooling | new follow-on | eliminate shell-owned card mutation |
| Add a dedicated `adl-pr-watch` binary if watch latency remains hot | new follow-on | only if measured need exists |
| Produce a command contract matrix test for every `pr.sh` subcommand | new follow-on | proves shim delegates instead of owning policy |
| Remove stale compatibility branches after the v0.91.7 window | v0.91.7 follow-on | requires operator approval |
| Promote GitHub/C-SDLC projection ownership into an ADR if accepted | ADR route | related to ADR candidate 0037 |

## Non-Claims

- This packet does not fully rewrite `pr.sh`; it lands the first split into
  sourced delegation and usage helpers.
- This packet does not remove the `pr.sh` entrypoint.
- This packet does not approve removing compatibility aliases.
- This packet does not claim all validation throughput work is complete.
- This packet does not change runtime/provider behavior.
- This packet does not replace issue cards, workflow-conductor, or closeout
  truth.

## Acceptance Check

This packet covers the required `#4481` architecture surfaces:

- current-state inventory of `pr.sh` responsibilities and failure modes
- target architecture for a stable shim over typed command modules
- migration map from current subcommands to typed binaries/modules
- compatibility policy and failure-mode policy
- validation and observability strategy
- routing table for existing fixes and follow-on implementation slices
- first implementation slice that shrinks `pr.sh` and proves helper-load
  observability, delegate compatibility, built-binary preference, and shell
  wrapper inventory freshness
