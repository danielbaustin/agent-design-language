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

## Active Session Registry

ADL needs a first-class local session registry. Until the registry command
exists, sessions should treat the following fields as the required coordination
shape for handoff notes and future registry records:

- `session_id` or thread identifier when known
- issue number or sprint umbrella
- branch
- worktree path
- current lifecycle stage
- PR number or URL when known
- do-not-touch paths
- watcher or janitor owner when one exists
- last meaningful update time
- known blockers

Future tooling should expose this through commands such as:

```text
adl session status
adl session claim
adl session heartbeat
adl session release
```

That implementation is intentionally follow-on work. This policy issue only
establishes the documented contract.

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

- implement the future session registry commands
- replace `workflow-conductor`
- replace issue cards or closeout truth
- permit tracked work on `main`
- make chat memory authoritative
