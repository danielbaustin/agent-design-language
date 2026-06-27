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

- Start every tracked issue through `workflow-conductor` and the repo-native
  `adl/tools/pr.sh` lifecycle.
- Keep the root checkout clean on `main`. Use root for inspection, doctor, and
  issue binding only.
- Before `pr run`, create or confirm the session-ledger claim required by
  `pr doctor`.
- After `pr run` binds the issue, create the issue-bound goal before editing.
- Make tracked implementation, janitor, finish, and closeout edits only in the
  issue worktree.
- Keep `SIP`, `STP`, `SPP`, `VPP`, `SRP`, and `SOR` truthful. Design-time
  cards must be ready before execution. `SRP` records review truth. `SOR`
  records execution, validation, integration, and closeout truth.
- Use watcher ownership for real wait states. A PR waiting on CI, review,
  mergeability, dependency truth, or operator decision is not abandoned; it is
  watcher-owned until it routes to `pr-janitor`, `pr-closeout`, human review, or
  the next issue.
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

Use `adl/tools/pr.sh watch <issue-or-pr> --json` when an issue or PR enters a
wait state. Preserve the watcher packet or a concise summary in the issue
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

## Prep-Scout Routing

Use a prep scout when all of these are true:

- the current issue is already waiting truthfully;
- the operator wants the next issue prepared while waiting;
- root is clean on `main`;
- the candidate issue is concrete.

The prep scout may inspect issues, cards, worktrees, PR state, and session
ledger claims. It may run `pr.sh doctor --mode ready --json`. It must stop with
one of: `ready`, `blocked`, `collision`, or `needs_operator`.

The promotion rule is explicit: once the operator or conductor selects the
candidate for execution, leave prep-scout mode and use the normal session claim
plus `pr.sh run <issue>` path.

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
- prefer fresh built owner binaries in the current or primary checkout;
- prefer matching owner binaries on `PATH`;
- use Cargo fallback only when the issue explicitly opts into that compatibility
  behavior.

If an owner binary is missing and fallback is disabled, fail closed and record
the tooling bug or setup gap. Do not hide the failure behind ad hoc wrapper
scripts.

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
- replace `workflow-conductor`, issue cards, or PR closeout;
- turn watcher or prep-scout roles into implementers;
- claim the scheduler is autonomous;
- claim every old skill document is already perfect;
- close the rescue sprint by itself.
