# v0.91.5 First Internal Review Remediation Queue

## Metadata

- Milestone: `v0.91.5`
- Source review issue: `#3576`
- Queue umbrella: `#3899`
- Queue date: `2026-06-16`
- Queue status: `historical_closed`
- Doctor readiness: `#3899` returned `ready_status: PASS` on `2026-06-17`

## Queue Summary

- The first v0.91.5 internal review produced eight routed remediation issues:
  `#3891` through `#3898`.
- `#3899` closed on `2026-06-17` after all child issues closed.
- `#3574` remains the canonical Sprint 4 umbrella, but this queue is now a
  historical first remediation tranche rather than the current active control
  surface.
- The queue remains intentionally execution-focused historical evidence and
  does not claim v0.91.5 release readiness.

## Current Authoritative Successor Surfaces

For current release-tail and review truth, use:

- `docs/milestones/v0.91.5/SPRINT_v0.91.5.md`
- `docs/milestones/v0.91.5/QUALITY_GATE_v0.91.5.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP14_QUALITY_GATE_APPLICATION_2026-06-17.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_WP15_DOCS_REVIEW_ALIGNMENT_2026-06-17.md`
- `docs/milestones/v0.91.5/review/internal_review/V0915_SECOND_PASS_INTERNAL_REVIEW_PLAN_2026-06-17.md`

## Final Child State (Historical)

- `#3891` is merged.
- `#3892` is merged and closed out after PR `#3900`.
- `#3893` is closed out.
- `#3894` and `#3895` are closed out.
- `#3896` is merged via PR `#3907` and is now closed out.
- `#3897` is merged via PR `#3905` and is now closed out.
- `#3898` is merged via PR `#3906` and is now closed out.
- `#3899` is closed as the completed first remediation umbrella.
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

These findings did not justify widening beyond `#3891-#3899`, but they did mean
the tooling tranche under `#3896-#3898` had to absorb adapter/bootstrap/auth UX
hardening rather than treating the original three issue titles as exhaustive.

## Tooling Problems Captured for Remediation

The current retained tooling problem set for the `#3896-#3898` tranche is:

1. new worktrees can miss prompt-template scaffolding under
   `docs/templates/prompts`
2. new worktrees can miss repo-local `adl/tools` wrappers needed by the normal
   workflow
3. issue-mode binding can preserve too little of the repo-local helper
   execution environment, so later steps fail after the worktree is already
   bound
4. `pr.sh issue` commands do not always inherit usable GitHub auth from the
   local authenticated environment
5. issue-body validation surfaces strict missing-section failures too late for
   smooth operator repair
6. `run` can bind onto a stale baseline missing prerequisite in-flight outputs
   without warning early enough
7. the adapter still relies on manual bridge-repair knowledge for repeatable
   worktree startup
8. `pr finish` lacks supported finish-validation routing for the
   observability-source surface touched in `#3896`
9. `pr finish` lacks supported finish-validation routing for the markdown-AST
   tooling surface plus focused tests touched in `#3898`
10. truthful emergency publication after finish-path failure is not yet a
    first-class workflow path

Keep these as explicit historical remediation inputs and reference them when
later Sprint 4 review/remediation work needs the first-pass execution context.

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
    validation UX, stale-baseline warning findings, incomplete `pr finish`
    lane classification, and missing truthful emergency-publication path
    surfaced during live execution.
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
