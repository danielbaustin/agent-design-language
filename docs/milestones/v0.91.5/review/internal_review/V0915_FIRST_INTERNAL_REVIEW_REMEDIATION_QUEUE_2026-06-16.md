# v0.91.5 First Internal Review Remediation Queue

## Metadata

- Milestone: `v0.91.5`
- Source review issue: `#3576`
- Queue umbrella: `#3899`
- Queue date: `2026-06-16`
- Queue status: `active_execution`
- Doctor readiness: `#3899` returned `ready_status: PASS` on `2026-06-17`

## Queue Summary

- The first v0.91.5 internal review produced eight routed remediation issues:
  `#3891` through `#3898`.
- `#3574` remains the canonical Sprint 4 umbrella, but this queue is the
  bounded first remediation tranche that should run before the remaining
  closeout-tail work resumes.
- The queue is intentionally execution-focused and does not claim v0.91.5
  release readiness.

## Current In-Flight State

- `#3891` is merged.
- `#3892` is merged and closed out after PR `#3900`.
- `#3893` is closed out.
- `#3894` and `#3895` are published as draft PRs.
- `#3899` is structurally ready to run, but its doctor preflight remains
  blocked by the existing open tools-wave PR, which is expected and should be
  preserved rather than treated as corruption.
- No Rust refactoring work is included in this queue.

## Supplemental Tooling Remediation Added During Execution

Live execution of the remediation flow surfaced additional adapter/worktree
findings that belong inside this same bounded queue:

- worktree bootstrap must preserve prompt-template scaffolding under
  `docs/templates/prompts`
- worktree bootstrap must preserve repo-local `adl/tools` wrappers needed by
  the normal workflow
- issue-mode binding must fail earlier or repair earlier when repo-local helper
  assumptions are missing
- `pr.sh issue` operations should inherit usable GitHub auth from the local
  authenticated environment without requiring manual `GITHUB_TOKEN` wrapping
- issue-body validation should surface required missing sections such as
  `Issue-Graph Notes` earlier and more canonically
- `run` should warn earlier when binding onto a stale baseline missing
  prerequisite in-flight issue outputs
- `pr finish` should classify observability-source changes such as
  `adl/src/cli/observability.rs` and tooling-command sources such as
  `adl/src/cli/tooling_cmd/markdown_ast_edit.rs` into valid finish-validation lanes so normal
  publication does not fail after focused proof is already complete

These findings do not justify widening beyond `#3891-#3899`, but they do mean
the tooling tranche under `#3896-#3898` must absorb adapter/bootstrap/auth UX
hardening rather than treating the original three issue titles as exhaustive.

## Ordered Execution Plan

1. `#3891` and `#3892`
   - Reason: validation-truth and delegation-proof surfaces affect how later
     remediation and closeout truth should be interpreted.
2. `#3893`
   - Reason: toolkit sprint closeout and card truth should be repaired before
     downstream review/remediation packets depend on them.
3. `#3894` and `#3895`
   - Reason: docs/evidence cleanup is parallel-safe once lifecycle truth is
     repaired.
4. `#3896`, `#3897`, and `#3898`
  - Reason: tooling fixes are parallel-safe when file ownership remains
    disjoint.
  - Expanded scope inside the existing tranche: absorb the operator-reported
    worktree bootstrap, local bridge, GitHub auth handoff, issue-body
    validation UX, and stale-baseline warning findings surfaced during live
    execution.
5. `#3899` closeout
   - Close the umbrella only after child issues are closed or explicitly
     rerouted.

## Parallel-Safe Grouping

- Group A: `#3894` and `#3895`
- Group B: `#3896`, `#3897`, and `#3898`

Do not parallelize `#3893` ahead of `#3891` / `#3892`, and do not close the
umbrella while child issues are still active.

## Entry Conditions

- Use `workflow-conductor` for every child issue.
- Use the active prompt-template registry and normal issue-mode worktree flow.
- Preserve existing in-flight state for `#3892` / PR `#3900`.
- Run bounded pre-PR review for each child issue before publication.
- Keep validation focused on the touched surface; do not widen into broad
  milestone validation inside child remediations.

## Exclusions

- Rust refactoring mini-sprint `#3745` remains separate and not started.
- External / third-party review `#3580` is not part of this queue.
- Final v0.92 preflight `#3577`, next-milestone planning `#3581`, and release
  ceremony `#3578` are downstream of this queue.

## Handoff Notes

- Use this queue packet together with:
  - `docs/milestones/v0.91.5/review/internal_review/V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md`
  - issue umbrella `#3899`
- Treat this packet as retained sequencing truth only. Child issues still own
  their individual implementation, validation, review, and closeout records.
