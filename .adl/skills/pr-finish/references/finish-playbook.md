# PR Finish Playbook

Use this file after the main skill triggers and you are ready to execute `pr finish`.

## Purpose

Close out a bounded issue truthfully by validating the output record, staging the intended paths, and creating or updating the reviewable PR.

## Checklist

- resolve the target issue/branch/worktree
- confirm the output record is present and no longer a bootstrap stub
- confirm the staged paths are explicit and issue-scoped
- prefer `adl/tools/pr.sh finish`
- record exactly what validation ran
- stop after PR publication/update

## Failure Handling

If finish cannot proceed:
- report the missing or inconsistent finish input
- report whether the output record was present
- stop without widening scope
