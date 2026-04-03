# Issue Bootstrap Playbook

Use this file after the main skill triggers and you are ready to execute the issue-bootstrap step.

Planning basis:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
- this skill bundle's `SKILL.md` and reference files

If the repo relocates those docs, follow the relocated canonical copies instead of these exact paths.

## Purpose

Turn a requested issue into a valid tracked bootstrap state for the next lifecycle step.

The bootstrap result should leave the repository with:
- a GitHub issue
- a canonical local source issue prompt
- a root task bundle with `stp.md`, `sip.md`, and `sor.md`
- validation evidence that qualitative card review can begin

This step is mechanical only.

It must not:
- qualitatively rewrite STP or SIP beyond standard bootstrap generation
- create a branch
- create a worktree
- begin implementation

## Execution Modes

### Mode A: Create And Bootstrap

Use this when the request starts from a title or new issue request.

Expected path:
1. create the GitHub issue
2. capture issue number and URL
3. generate or seed the canonical source issue prompt
4. ensure the root task bundle exists for the new issue
5. validate the result
6. stop

### Mode B: Bootstrap Existing Issue

Use this when the issue already exists and only needs the local bootstrap bundle.

Expected path:
1. resolve issue metadata using the repo's standard flow
2. infer or confirm slug/version
3. ensure the canonical source issue prompt exists
4. run the init/bootstrap phase
5. validate the result
6. stop

## Validation Checklist

Confirm all of these before returning success:
- issue number exists
- issue URL exists for newly created issues
- source issue prompt exists at the canonical path
- task-bundle directory exists
- `stp.md` exists
- `sip.md` exists
- `sor.md` exists
- compatibility card links exist if the repo expects them
- no branch was created
- no worktree was created
- the next step is clearly identified as qualitative card review

## Review Questions

Ask these questions after bootstrap:
- Did the issue land in the expected version/scope path?
- Does the slug match the canonical path layout?
- Do the source prompt and bundle point at the same issue identity?
- Did any expected bootstrap surface fail to appear?
- Did the operation accidentally broaden into start/worktree behavior?
- Did the process remain mechanical rather than silently editing review content?

## Parallel Safety

Parallel execution is safe only when invocations do not share issue targets or bundle paths.

Safe examples:
- create/bootstrap issue A and issue B at the same time
- bootstrap separate existing issues with different task-bundle roots

Unsafe examples:
- two bootstrap runs targeting the same issue number
- a bootstrap run overlapping with issue-mode `pr run` for the same issue

## Current Compatibility Note

The skill should prefer the Rust-owned control-plane path when available.

Current command truth:
- use `adl pr create` or `adl/tools/pr.sh create` when a new issue must be created
- use `adl pr init` or `adl/tools/pr.sh init` when the issue already exists
- hand off to qualitative review after bootstrap
- only after review does issue-mode `pr run` bind branch and worktree context

Do not teach `pr start` as the public execution binder for this workflow.

## Failure Handling

If the process fails:
- report whether the GitHub issue was created
- report whether the source prompt exists
- report whether the bundle directory exists
- report which of `stp.md`, `sip.md`, `sor.md` exist
- report whether any compatibility links were created
- stop without attempting branch/worktree repair
- hand off to review or human follow-up only after the missing bootstrap surfaces are made explicit
