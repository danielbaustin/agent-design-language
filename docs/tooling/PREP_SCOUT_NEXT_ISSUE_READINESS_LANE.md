# Prep-Scout Next-Issue Readiness Lane

## Purpose

The prep-scout lane exists so one session can prepare the next issue while the
current issue is in PR wait, review wait, CI wait, janitor wait, or
merge-needs-closeout wait.

The lane is preparation-only. It reduces cold-start time for the next issue,
but it does not start implementation early.

## When To Use It

Use the lane when all of the following are true:

- the current issue is already in a truthful wait state such as
  `pr_waiting`, `janitor_active`, or `merged_needs_closeout`
- the operator wants the next issue prepared without widening the current
  closeout lane
- the primary checkout is clean on `main`
- the next issue is a concrete candidate rather than a vague backlog browse

Do not use the lane when:

- the current issue still needs implementation work
- the next issue has already been bound for execution by another session
- the operator is really asking to begin the next issue, not just prepare it

## Allowed Surfaces

The prep scout may use:

- `workflow-conductor` for routing truth
- repo-native `bash adl/tools/pr.sh issue list|view|search`
- repo-native `bash adl/tools/pr.sh doctor <issue> --mode ready --json`
- root-checkout inspection commands while the root checkout remains clean on
  `main`
- issue/task-bundle reads for the candidate issue
- session-ledger reads to detect collisions

The prep scout may also note that a candidate would need:

- `pr init` because the task bundle is missing
- card-editor repair because readiness defects are present
- operator judgment because candidate choice or dependency truth is ambiguous

## Prohibited Actions

The prep scout must not:

- run `pr run` for the candidate issue
- bind or create an implementation worktree for the candidate issue
- start product, tooling, docs, or test implementation for the candidate issue
- publish or update a PR for the candidate issue
- hide a tooling limitation behind manual git surgery or raw `gh`

## Current Tooling Boundary

Current repo-native tooling cleanly supports read-only scouting and readiness
classification.

Current repo-native tooling does not yet provide a first-class prep-only bind
surface for card repair that preserves:

- clean-root `main` discipline
- no-implementation-start semantics
- no-hidden-manual-worktree fallback

Because that surface is missing, a prep scout should treat prep-time mutation as
`needs_operator` unless the operator explicitly promotes the candidate into the
normal execution path.

## Standard Flow

1. Confirm the current issue is in a wait-capable state.
2. Confirm `git status --short --branch` is clean on root `main`.
3. Use `workflow-conductor` to route the current issue and preserve the waiting
   lane truth separately from the prep lane.
4. Inspect candidate issues with repo-native issue commands.
5. Check for collision evidence in the shared session ledger and existing
   worktree/PR state.
6. Run `pr.sh doctor --mode ready --json` for the candidate when a task bundle
   already exists.
7. Emit one bounded handoff result and stop.

## Handoff States

Use one of these terminal handoff states:

- `ready`
  - The candidate is structurally ready for normal execution.
  - Include the exact next command, normally
    `bash adl/tools/pr.sh run <issue>`.
- `blocked`
  - The candidate is not executable yet because readiness defects or hard
    dependencies are present.
  - Include the blocking evidence and the next repair route.
- `collision`
  - Another session, worktree, branch, or PR state already owns the candidate.
  - Include the owning evidence and the do-not-touch route.
- `needs_operator`
  - More than one candidate is plausible, or prep would require mutation that
    current repo-native tooling cannot express safely as preparation-only work.

## Suggested Handoff Shape

```yaml
prep_scout_handoff:
  candidate_issue: 0
  state: ready | blocked | collision | needs_operator
  current_issue_wait_state: pr_waiting | janitor_active | merged_needs_closeout
  evidence:
    - "repo-native issue view/list result"
    - "doctor/readiness result"
    - "session-ledger or worktree collision result"
  next_command: "bash adl/tools/pr.sh run <issue>"
  notes:
    - "record any tooling gap explicitly"
```

## Observed Proof Examples

This issue proved the handoff contract against live repo-native readiness
surfaces without starting implementation for the candidate issues:

- `#4438` -> `collision`
  - another session already owns the active claim and bound worktree, so the
    prep scout must stop instead of duplicating ownership
- `#4534` -> `blocked`
  - stale session-claim residue requires manual inspection before execution can
    resume truthfully
- `#4530` -> `ready`
  - the issue is structurally ready for normal execution; the next step is the
    standard session claim plus `bash adl/tools/pr.sh run 4530`, not early
    implementation from the prep lane

These examples are intentionally workflow-level proof only. They do not claim
merge, closeout, or autonomous issue execution.

## Sprint Execution Packet Integration

When a sprint or mini-sprint expects closeout-wait compression, its Sprint
Execution Packet should name:

- the candidate next-issue queue
- whether the prep lane is read-only or blocked pending tooling support
- the owner or watcher responsible for the prep handoff
- the promotion rule from prep to normal `pr run`

## Non-Claims

- This lane is not autonomous issue execution.
- This lane does not weaken issue-start gates from `#4435`.
- This lane does not authorize tracked work on `main`.
- This lane does not claim a prep-only mutation surface already exists.
