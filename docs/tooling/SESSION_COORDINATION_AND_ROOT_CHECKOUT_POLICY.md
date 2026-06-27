# Session Coordination And Root Checkout Policy

## Purpose

ADL sessions often run in parallel. They share the same Git repository, issue
queue, local worktrees, and operator attention, but they do not automatically
share chat history or intent. This policy makes the root checkout and session
handoff rules explicit so one session does not strand another on the wrong
branch, overwrite active work, or hide important workflow state in memory.

## Current Authority

This document clarifies the existing workflow rules in:

- `AGENTS.md`
- `docs/tooling/C_SDLC_RESCUE_SPRINT_OPERATING_CONTRACT.md`
- `docs/onboarding.md`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`
- the `workflow-conductor` and `pr-run` skill contracts

If this document conflicts with `AGENTS.md`, `AGENTS.md` wins until both files
are updated together.

## Root Checkout Ownership

The primary checkout is the repository root, for example:

```text
/Users/daniel/git/agent-design-language
```

The primary checkout must normally stay on clean `main`.

Allowed primary-checkout uses:

- read-only inspection
- issue creation/bootstrap
- `pr ready` / `pr doctor`
- prep-scout issue inspection/readiness checks for a separate next-issue lane
  while the current issue is in a truthful wait state
- issue-mode `pr run` binding when `main` is clean
- fast-forwarding `main`
- checking root/worktree state before routing

Disallowed primary-checkout uses:

- tracked implementation edits
- janitor repairs to PR branches
- finish staging for an issue branch
- leaving the root checkout on a feature branch
- parking untracked issue artifacts in root when a bound worktree exists

After an issue is bound, tracked edits happen in the bound issue worktree.
The bound worktree's local `.adl/<version>/tasks/...` bundle is the active
issue-local execution surface for normal issue work. Materialized
`.adl/<version>/sprints/...` packet copies in the worktree are convenience
mirrors for local context, not silent replacements for the primary checkout's
canonical sprint record. If the worktree-local issue identity disagrees with
the primary checkout for the same issue, stop and repair the mismatch instead
of guessing which copy is right. Root-only `.adl` state remains bootstrap,
coordination, and sprint-truth context rather than a hidden per-issue live
authority during execution.

Prep-scout exception:

- a prep scout may use the root checkout for read-only next-issue inspection
  and readiness classification while another issue is waiting
- the prep scout must not convert that root-checkout preparation pass into
  tracked implementation or hidden candidate-issue mutation on `main`
- if a candidate would require mutation before it can be called ready and there
  is no proven prep-only repo-native bind surface, stop as `needs_operator` and
  record the tooling gap instead of improvising a manual fallback
- if the candidate issue is promoted into execution, leave prep-scout mode and
  use the normal session claim plus `adl/tools/pr.sh run <issue>` path before
  any tracked edits occur

## Required Startup Check

Before starting or resuming tracked issue work, a session must check:

```bash
git status --short --branch
git worktree list --porcelain
```

Expected root state:

```text
## main...origin/main
```

If the primary checkout is not on `main`, has tracked changes, or is occupied by
an issue branch, stop before implementation. Route the recovery through
`workflow-conductor` and repo-native `pr run` or `pr doctor` evidence when the
issue/worktree can be identified. Use only the narrowest manual fallback needed
to preserve work in an issue worktree, restore the primary checkout to clean
`main`, and record what moved where.

If a broad process check is needed, use the permission-safe process helper from
`docs/tooling/PERMISSION_SAFE_PROCESS_STATUS.md`; do not use broad `ps`,
`pgrep`, or `lsof` scans as workflow control.

## Polis Occupancy Ledger

ADL needs a first-class local occupancy ledger. The immediate C-SDLC use case is
session coordination across issues, branches, PRs, and worktrees, but the
underlying concept is broader: agents and sessions claim bounded authority over
shared polis resources.

The general shape is:

```text
agent/session/role
claims authority over
resource/surface/task
for a bounded purpose
under a policy
with heartbeat, expiry, handoff, and audit trail
```

For the software guild, resources are usually C-SDLC issues, PRs, branches, and
worktrees. Future guilds may use the same claim model for research threads,
publication drafts, model evaluations, memory-palace rooms, governance
proposals, compute budgets, local machines, or Observatory surfaces.

The first implemented ledger surface is:

```bash
adl session status [--ledger <path>] [--json]
adl session claim --session-id <id> --owner <name> --resource <kind:id> --purpose <text> [--issue <n>] [--pr <n>] [--branch <name>] [--worktree <path>] [--policy-ref <path>] [--lifecycle-phase <phase>] [--mode active|watching|paused] [--ttl-secs <n>] [--do-not-touch <path>]... [--blocker <text>]... [--ledger <path>] [--json]
adl session heartbeat --claim-id <id> [--ttl-secs <n>] [--ledger <path>] [--json]
adl session release --claim-id <id> [--reason <text>] [--ledger <path>] [--json]
```

Default local ledger path:

```text
.adl/session-ledger/ledger.json
```

The default path is rooted at the primary checkout, not the current linked
worktree. Sessions launched from `.worktrees/adl-wp-*` and sessions launched
from the root checkout must therefore converge on the same shared
`.adl/session-ledger/ledger.json` file unless an explicit `--ledger` override
is supplied.

The ledger records:

- `session_id` or thread identifier when known
- owner/account/agent label
- generic `resource.kind` and `resource.id`
- purpose
- claim mode: `active`, `watching`, `paused`, `stale`, or `released`
- issue number or sprint umbrella when the resource is C-SDLC work
- branch
- worktree path
- current lifecycle stage
- PR number or URL when known
- do-not-touch paths
- watcher or janitor owner when one exists
- heartbeat, expiry, and release timestamps
- known blockers

The ledger is coordination infrastructure, not final authority. GitHub issue/PR
state, tracked cards, and closeout records remain authoritative for lifecycle
truth. A stale ledger claim must block blind writes; it does not authorize
destructive cleanup by itself.

Current rescue-sprint practice uses the ledger to transfer wait states rather
than abandon them. When an issue enters CI, review, mergeability, dependency, or
operator-decision wait, the active session should release or hand off the
implementation claim and create a watcher claim that names the issue, PR,
branch, worktree, and expected next skill. When the watcher routes a blocker to
`pr-janitor` or a merged PR to `pr-closeout`, the claim should move with that
route instead of leaving duplicate active ownership behind.

Mutating commands acquire a short-lived sibling lock file next to the selected
ledger path, such as `.adl/session-ledger/ledger.json.lock`, before loading and
rewriting the ledger. A leftover lock file means a previous process may have
stopped mid-mutation and should be inspected before removal.

## Broadcast Notes

When a session changes shared workflow state, it must leave a short durable note
in the relevant issue, sprint packet, PR, or closeout record. Examples:

- root checkout repaired
- feature branch moved from root into `.worktrees/...`
- issue is active in another session
- issue is waiting on CI, review, or watcher
- raw GitHub fallback was used because repo-native tooling failed
- lifecycle wrapper stalled or failed and a bounded fallback was used

Broadcast notes should be factual, brief, and free of secrets. They should name
the issue, branch, worktree, and next expected owner/action.

For v0.91.6 rescue-sprint work, broadcast notes should also name whether the
state is active execution, watcher-owned wait, janitor repair, prep-scout
handoff, or closeout. This keeps resumed sessions from guessing whether a draft
PR is healthy, blocked, or abandoned.

## Collision Handling

When another session appears to own an issue or worktree:

1. Do not start duplicate implementation work.
2. Inspect the issue, PR, branch, and worktree state.
3. If the state is healthy, leave it alone or watch it.
4. If the state is stale or broken, record the evidence and route through
   `workflow-conductor`, `pr-janitor`, or `pr-closeout` as appropriate.
5. If root is occupied by that work, route through `workflow-conductor` and
   repo-native worktree evidence first. Use manual preservation only as a
   bounded fallback to move the work into an issue worktree before restoring
   root to clean `main`.

Prep-scout-specific collision rule:

- if the candidate issue already has an active session claim, open PR, or bound
  worktree owned by another session, classify the handoff as `collision`
  instead of beginning duplicate preparation

## Tooling Failure Handling

If a repo-native lifecycle command fails or hangs:

- stop the command rather than waiting indefinitely
- verify whether it partially created an issue, worktree, PR, or local bundle
- record the failure in the issue or a remediation issue
- use the narrowest fallback needed to preserve root checkout safety
- do not normalize the fallback into the preferred workflow

This rule exists so emergency cleanup does not become a second, undocumented
workflow.

## Non-Goals

This policy does not:

- replace `workflow-conductor`
- replace issue cards or closeout truth
- permit tracked work on `main`
- make chat memory authoritative
- solve all future polis governance or non-software guild occupancy rules
