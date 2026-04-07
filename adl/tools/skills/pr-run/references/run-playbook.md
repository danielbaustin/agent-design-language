# PR Run Playbook

Use this file after the main skill triggers and you are ready to execute an issue.

Planning basis:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
- `/Users/daniel/git/agent-design-language/.adl/docs/v0.87planning/promoted/PR_TOOLING_SKILLS.md`

If the repo relocates those docs, follow the relocated canonical copies instead of these exact paths.

## Purpose

Execute a prepared issue after readiness review by binding or confirming its execution branch/worktree, doing the bounded work, validating it, and recording the result truthfully.

This step is automatable but writes real repo state.

It may:
- inspect and confirm readiness
- bind or confirm branch/worktree execution context
- edit issue-scoped code/docs/tests/artifacts
- run bounded validation
- update the output record

It must not:
- bootstrap a missing issue from scratch
- replace qualitative issue review as its own separate concern
- silently merge or close the PR

## Target Resolution

Resolve the most concrete available target in this order:
1. explicit issue number
2. explicit task-bundle path
3. explicit worktree path
4. explicit branch

If targets disagree materially, report `blocked`.

## Doctor Gate

Before implementation:
- prefer an explicit doctor result if available
- otherwise run repo-native readiness checks

Decision rule:
- if execution readiness is `blocked`, stop
- if execution readiness is `ready` or `ready_with_repairs`, execution may proceed
- if preflight is blocked, treat that as a separate scheduling gate

Default stance:
- stop on a blocked preflight gate unless the caller explicitly wants to run under override

## Binding Checklist

Check or establish:
- issue id and slug coherence
- source prompt presence
- STP/SIP/SOR presence
- whether execution-time binding should create the branch/worktree now
- branch-to-issue traceability if a branch already exists
- worktree branch match if a worktree already exists
- whether `pr run` should create the branch/worktree now or reuse existing bound state from compatibility/prior execution

Late-binding rule:
- do not require a branch/worktree to exist before execution time
- absence of branch/worktree is normal for a prepared issue
- prefer creating or reusing the execution context only when the run actually begins

Post-bind materialization rule:
- after binding, the worktree-local execution bundle must exist
- verify worktree-local `stp.md`, `sip.md`, and `sor.md`
- if any are missing, treat that as a run failure or blocked execution state rather than a soft warning
- do not continue into implementation on root-only bundle state once the worktree has been bound

## Execution Checklist

Read and use:
- source prompt
- STP
- SIP
- current SOR/output card

Then:
- perform only the work required by the issue
- keep edits bounded to the issue's acceptance criteria
- record follow-on discoveries instead of silently expanding scope

## Validation Checklist

Run the smallest truthful validation set that matches the changed surface:
- formatter or lint checks when relevant
- focused tests for the touched subsystem
- targeted proof/demo commands when required by the issue

Always report:
- what ran
- what it verified
- what was intentionally not run

## Output Card Rules

The execution record should answer:
- what was changed
- where the work lives now
- what was validated
- whether the issue is still in progress or done
- what the correct next step is

For `pr_open` state:
- tracked branch paths are not "worktree-only"
- reserve `worktree_only` for artifacts stranded outside the tracked repo branch surface

For execution binding:
- record explicitly whether worktree-local `stp.md`, `sip.md`, and `sor.md` were present after binding
- if materialization was missing or had to be repaired, surface that in findings and handoff guidance

## Failure Handling

If execution fails:
- report which target was attempted
- report which checks were actually performed
- record partial outputs truthfully
- stop without widening scope
