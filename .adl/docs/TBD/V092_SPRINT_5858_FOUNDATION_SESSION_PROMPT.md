# v0.92 Sprint #5858 Session Prompt

Use this prompt to start the Foundation and Throughput sprint session.

```text
You own v0.92 sprint coordination issue #5858, Foundation and Throughput.

Read the repository root AGENTS.md before doing anything else. Follow the final
typed C-SDLC v2 authority. Never write tracked work on main, never use
/private/tmp, never invoke sunset wrappers, and never expose credentials.

The umbrella coordinates child work; it does not own child implementation.
Every child keeps its own issue-bound worktree, claim, session goal, validation,
review, PR, merge, finish, and cleanup truth. Do not edit child product paths
from the #5858 umbrella worktree.

Before execution:

1. Verify WP-01 issue #5817 is merged and its merge commit is ancestral to the
   current main revision. Do not recreate sprint or child records if it is not.
2. Verify the root checkout is clean on main and inspect active worktrees.
3. Read issue #5858 and:
   - .csdlc/issues/5858/
   - .csdlc/prepared/issues/5858/sprint-execution-packet.md
   - .csdlc/prepared/issues/5858/sprint-execution-packet.yaml
   - docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
4. Run typed doctor for #5858 and each child. The canonical
   .csdlc/issues/<issue> records are authoritative. Do not recreate their six
   cards because an older helper looks for sunset .adl task bundles.
5. Use sprint-conductor for coordination, but route every implementation
   through the child's typed v2 lifecycle.

Exact child wave:

- #5818, WP-01B: activate canonical v0.92 docs and version surfaces
- #5819, WP-02: perform the Agent Logic repository migration
- #5812: bounded Freedom Gate Clippy cleanup coordinated with CI
- #5801, WP-02A: repair and simplify CI/PVF/coverage
- #5853, WP-02B: run the bounded post-migration build experiment
- #5822, WP-05: C-SDLC estimation and cycle-time reduction
- #5823, WP-06: remote validation/build runner
- #5824, WP-07: prompt-card enum typing

Serial gates:

- #5818 before #5819
- #5819 before #5801 and #5853
- #5801 before #5853, #5822, and #5823
- #5822 before #5824

Safe parallel work after #5801:

- #5822 and #5823 may execute in separate child worktrees.
- #5812 may execute as a bounded low-collision repair coordinated with #5801.
- #5853 starts only after migration, CI, budget, and runner-access gates pass.

Start with #5818. For each dependency-ready child: bind its worktree, create an
issue-bound goal before implementation, execute the complete useful outcome,
run the smallest proving validation, obtain independent pre-PR review, fix all
actionable findings, and publish a PR containing `Closes #<child>`.

Do not stop because a healthy PR is waiting. Give waiting PRs to a watcher and
continue another declared safe lane. Do not force parallel work when paths or
proof surfaces collide; collapse that lane to serial and record why.

Maintain:

- .csdlc/evidence/5858/activity.jsonl
- .csdlc/evidence/5858/sprint-review.md

Close #5858 only after every child has truthful terminal state and the integrated
sprint review has no unresolved actionable finding. Closeout must never block
independent useful work in another ready lane.
```
