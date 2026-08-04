# Issue 4741 design

## Decision

Make every Unity Observatory proof launch declare one of three modes before it
does work:

- `open_editor`: use the already-open intended project through an approved
  editor-mediated proof path;
- `fresh_batch`: launch one isolated staged copy when no conflicting editor owns
  that project;
- `skipped_fail_closed`: stop with a precise reason when neither mode is safe.

The wrapper must use repository-installed binaries, permission-safe process
status, `/Volumes/FastWork` or issue-local `.adl/` staging, and log-progress
watchdogs. It must not depend on a fixed total-runtime ceiling.

## Ownership boundary

Issue #4741 owns editor/project liveness classification, launch-mode selection,
staging isolation, progress monitoring, cleanup, and focused wrapper tests.

Issue #4739 owns Unity-MCP project and endpoint alignment. Issue #5332 owns the
ILPP `GetDomainName: -1` retry-loop cause, signature, threshold, and classifier.
#4741 may consume and route a generic external startup-classifier result but
must not encode, diagnose, or repair ILPP.

Scene building, asset fallback, runtime contract semantics, investor rendering,
and walkthrough capture are outside this issue.

## Preserved predecessor work

An older dirty #4741 worktree contains useful candidate wrapper changes mixed
with Unity scene code. It remains untouched. Execution should selectively port
only wrapper liveness, staging, progress-watchdog, and focused-test changes.

## Proof design

1. Resolve the intended project and repository-installed Unity/ADL binaries.
2. Determine whether that exact project is already owned by an editor.
3. Select and print one execution mode before launching or calling proof.
4. For fresh batch mode, copy only the required project into an approved
   writable staging root and isolate mutable Unity/.NET scratch state.
5. Track semantic log progress and fail on bounded idle, crash, licensing,
   readonly-database-without-progress, or a generic external startup-classifier
   result.
6. Preserve one concise result packet containing mode, project, editor version,
   progress classifier, log reference, and terminal outcome.
7. Register the issue-owned wrapper paths in the validation selector and prove
   the focused lane selection without widening to the full Unity proof matrix.
