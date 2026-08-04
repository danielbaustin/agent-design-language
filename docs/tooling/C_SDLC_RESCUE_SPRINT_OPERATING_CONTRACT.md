# C-SDLC Rescue Sprint Operating Contract

## Purpose

This contract is the v0.91.6 rescue-sprint operating guide for sessions that
need to move quickly without breaking ADL workflow truth. It explains how the
current conductor, watcher, janitor, prep-scout, scheduler, prompt-card,
validation, and binary-first command surfaces fit together.

The goal is simple: a session should not rediscover these rules by failing a
PR, writing on `main`, abandoning a wait state, or running a broad validation
lane by accident.

## Current Contract

- Start every tracked issue through the typed C-SDLC v2 skills and binaries,
  after confirming `csdlc-install resolve` selects `v2`.
- Keep the root checkout clean on `main`. Use root for inspection, doctor, and
  issue binding only.
- Before `csdlc-bind`, create or confirm the typed claim required by
  `csdlc-doctor`.
- After `csdlc-bind` binds the issue, create the issue-bound goal before editing.
- Make tracked implementation, janitor, finish, and closeout edits only in the
  issue worktree.
- Keep `SIP`, `STP`, `SPP`, `VPP`, `SRP`, and `SOR` truthful. Design-time
  cards must be ready before execution. `SRP` records review truth. `SOR`
  records execution, validation, integration, and closeout truth.
- Use watcher ownership for real wait states. A PR waiting on CI, review,
  mergeability, dependency truth, or operator decision is not abandoned; it is
  watcher-owned until it routes to `pr-janitor`, `pr-closeout`, human review, or
  the next issue.
- Issue-bound PR publication must attach a watcher packet during `pr finish`.
  Disabling watcher attachment is fail-closed, PR inventory reports missing
  watcher packets, and closeout must record or update a terminal watcher
  disposition before the issue bundle is considered clean.
- Use prep scouts only for read-only next-issue readiness while the current
  issue is in a truthful wait state. Prep scouts do not bind worktrees, mutate
  cards, or start implementation.
- Treat the scheduler as advisory in v0.91.6. It can produce and consume plan
  artifacts, including Soak #1 advisory surfaces, but it does not run timed
  jobs, mutate GitHub, choose providers authoritatively, or conduct sprints.
- Prefer focused validation based on the changed surface. Do not let small docs
  or janitor issues expand into full coverage unless the path policy requires
  it.
- Workflow-critical ADL commands should resolve independent owner binaries
  before falling back to Cargo. `#4590` owns the binary-first command contract;
  normal rescue-sprint operation should not use hidden `cargo run` as the
  default command path.

## Wait-State Routing

Use the typed `csdlc-shepherd` route when an issue or PR enters a wait state.
Preserve the watcher packet or a concise summary in the issue
record, sprint packet, SRP, SOR, or closeout artifact.

Current routing keys come from the watch packet's top-level classification:

- `pr_open` or `checks_running`: keep watcher ownership.
- `checks_failed`, requested changes, or merge conflicts: route to
  `pr-janitor`.
- `checks_green_but_draft`: route to `pr-janitor` for draft-state transition.
- `checks_green`: preserve human-review or merge-authority handoff.
- `merged_pending_closeout` or `closeout_needed`: route to `pr-closeout`.
- `ready_for_run` or `blocked`: treat as pre-publication readiness truth and
  follow the packet's `next_skill`.

Watchers do not implement issue scope. They classify, route, retain evidence,
and stop.

For issue-bound PRs, watcher evidence is durable under the primary checkout at
`.adl/logs/issue-watcher/issue-<issue>/` so it survives issue-worktree pruning.
The PR attachment packet records the PR URL, expected state,
watcher input/prompt/log/pid paths, and terminal disposition once closeout
observes the completed tail. If the packet is missing for an issue-bound PR,
closeout stops and routes the defect back through finish, watch, or shepherd
repair instead of silently completing.

## Prep-Scout Routing

Use a prep scout when all of these are true:

- the current issue is already waiting truthfully;
- the operator wants the next issue prepared while waiting;
- root is clean on `main`;
- the candidate issue is concrete.

The prep scout may inspect issues, cards, worktrees, PR state, and session
ledger claims. It may run typed `csdlc-doctor` readiness. It must stop with
one of: `ready`, `blocked`, `collision`, or `needs_operator`.

The promotion rule is explicit: once the operator or conductor selects the
candidate for execution, leave prep-scout mode and use the normal typed claim
plus `csdlc-bind` request path.

## Scheduler Boundary

The v0.91.6 scheduler surface is a bounded planning and evidence component:

- `adl scheduler plan` is an operator-facing CLI surface.
- Successful scheduler execution writes one JSON plan record to stdout.
- Human-oriented diagnostics and parse failures belong on stderr.
- Soak #1 may consume a scheduler plan artifact.
- The scheduler remains non-authoritative for timed execution, GitHub mutation,
  provider selection, sprint conduction, and SSM command execution.

If a session needs autonomous scheduling behavior, that is follow-on work, not
something to infer from the v0.91.6 scheduler proof.

## Binary-First Command Path

Rescue-sprint commands should not discover at finish time that they need a
long Cargo build or a locked Cargo process. The expected command posture is:

- prefer explicit command-specific binary overrides;
- for `pr finish` in a bound issue worktree, prefer its fresh
  `adl/target/debug/adl-pr-finish` before a primary-checkout stable binary;
- prefer fresh built owner binaries in the current or primary checkout;
- prefer matching owner binaries on `PATH`;
- use Cargo fallback only when the issue explicitly opts into that compatibility
  behavior.

If an owner binary is missing and fallback is disabled, fail closed and record
the tooling bug or setup gap. Do not hide the failure behind ad hoc wrapper
scripts.

`ADL_PR_FINISH_BIN` remains the explicit repair override when the bound
worktree binary cannot be used. The resolver emits
`source=current_worktree_owner_bin` when it selects the normal bound-worktree
finish path. If production Rust inputs diverge and that binary is absent or
stale, finish exits `75` with rebuild and override guidance instead of using a
primary-checkout or `PATH` fallback.

## CSM Runtime Owner Binary Availability

`csm` is the runtime owner binary. Runtime liveness tests, daemon proofs,
continuity capsules, AWS signal proofs, CloudWatch heartbeat proofs, and final
runtime-coherence gates must not depend on incidental `target/` cache state.

Before a runtime proof wrapper invokes `csm`, use the repo-native availability
guard:

```sh
bash adl/tools/ensure_csm_binary.sh --json --out <proof-dir>/csm_binary_availability.json
```

The guard records source presence, selected binary path, provenance, action,
and warm-cache status. It reuses a trusted executable when available and restores
the binary with the repo-native `cargo build --manifest-path adl/Cargo.toml
--bin csm` path only when the binary is missing or stale. It does not vendor
compiled binaries into git and does not route runtime ownership back through
the `adl` tooling binary.

Runtime proof wrappers should source `adl/tools/csm_binary_availability.sh` and
call `adl_resolve_csm_binary <requested-bin> <evidence-json>` so the retained
proof packet shows whether the run reused or rebuilt `csm`.

For bounded missing-binary restoration proofs that must not move or hide the
primary checkout binary, set `ADL_CSM_BINARY_STRICT_REQUEST=1` with a fresh
`CARGO_TARGET_DIR`. In that mode the guard considers only the requested or
Cargo-target candidate and proves restoration there.

## Validation Posture

Use the smallest proof that matches the changed surface:

- docs-only changes: `git diff --check`, path/reference spot checks, and the
  milestone docs staleness check when milestone or review docs changed;
- prompt-card/template changes: values import/render/structure/schema checks;
- workflow-control changes: focused shell tests, stdout/stderr contract proof,
  and owner-binary resolution proof;
- runtime/product changes: the relevant runtime, demo, soak, or owner lane.

Validation records must say what ran locally, what CI will prove, and what was
not run.

## Non-Claims

This contract does not:

- authorize tracked work on `main`;
- replace typed v2 lifecycle routing, issue cards, or PR closeout;
- turn watcher or prep-scout roles into implementers;
- claim the scheduler is autonomous;
- claim every old skill document is already perfect;
- close the rescue sprint by itself.
