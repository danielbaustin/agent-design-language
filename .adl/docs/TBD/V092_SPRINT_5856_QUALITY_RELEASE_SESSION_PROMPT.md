# v0.92 Sprint #5856 Session Prompt

Use this prompt to start the Quality and Release-Tail sprint session.

```text
You own v0.92 sprint coordination issue #5856, Quality and Release Tail.

Read root AGENTS.md first. Use typed C-SDLC v2 only. Never write tracked work on
main, never use /private/tmp, and never treat GitHub closure alone as proof of a
completed child. The umbrella must not implement child changes directly.

This sprint is deliberately sequential. Its session may prepare future child
work while earlier milestones execute, but it must not start implementation or
claim acceptance before the declared gates pass.

Startup:

1. Verify WP-01 #5817 is merged and ancestral to current main.
2. Read issue #5856, .csdlc/issues/5856/, both Sprint Execution Packets under
   .csdlc/prepared/issues/5856/, the quality gate, and the v0.92 issue wave.
3. Run typed doctor for #5856 and each child. Use canonical .csdlc records; do
   not invoke pr-init merely because an obsolete helper cannot find .adl task
   bundles.
4. Audit readiness and dependencies without executing out of order.

WP-01 published the initialized child records under its own temporary publication
claim. After WP-01 releases that claim, create and register each real child
worktree, use typed `csdlc-bind --reacquire-request` to acquire the child's exact
issue-local paths, then run the normal bind and goal sequence. Do not assume the
bootstrap reservation is still active.

Exact child wave:

- #5786, WP-21: repository-wide code reduction cleanup
- #5841, WP-21A: Rust refactoring and maintainability pass
- #5842, WP-22: quality gate over every indexed v0.92 feature
- #5843, WP-23: canonical docs and release-truth pass
- #5846, WP-25: internal review
- #5847, WP-26: external or third-party review
- #5848, WP-27: remediate all review findings
- #5849, WP-28: next-milestone planning
- #5850, WP-28A: exact next-milestone closeout plan
- #5851, WP-29: next-milestone review pass
- #5852, WP-30: release ceremony

Strict order:

- #5786 before #5841
- #5841 before #5842
- #5842 before #5843
- #5843 before #5846
- #5846 through #5852 in dependency order

For each child when its gate opens: bind the child worktree, create its own goal,
complete the full useful outcome, validate the correct surface, obtain exact-head
independent review, fix every actionable finding, and publish with
`Closes #<child>`. Reviews must inspect actual code and evidence, not only cards.

Do not run broad tests for docs-only work. Do not skip focused proof for code or
release claims. Do not allow closeout ceremony to delay unrelated implementation,
but do not advance this sequential release tail until the preceding child truth
is complete.

Maintain:

- .csdlc/evidence/5856/activity.jsonl
- .csdlc/evidence/5856/sprint-review.md

Close #5856 only after the release ceremony and complete child rollup agree on
issue, PR, evidence, review, release, and residual-risk truth.
```
