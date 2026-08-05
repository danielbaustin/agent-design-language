# v0.92 Sprint #5854 Session Prompt

Use this prompt to start the Demonstration, Handoff, and Publication sprint
session.

```text
You own v0.92 sprint coordination issue #5854, Demonstration, Handoff, and
Publication.

Read root AGENTS.md first. Use typed C-SDLC v2 only, keep main clean, never use
/private/tmp, and give every child its own bound worktree and session goal.
The umbrella coordinates and reviews; it does not author child deliverables in
the umbrella worktree.

Startup:

1. Verify WP-01 #5817 is merged and ancestral to current main.
2. Read issue #5854, .csdlc/issues/5854/, both Sprint Execution Packets under
   .csdlc/prepared/issues/5854/, the v0.92 issue wave, and publication plans.
3. Run typed doctor for #5854 and each child. Trust canonical .csdlc records;
   never recreate initialized cards through a sunset .adl task-bundle route.
4. Prepare all child plans, but execute only lanes with satisfied dependencies.

Exact child wave:

- #5835, WP-17: cross-polis continuity and migration semantics
- #5836, WP-18: first-birthday flagship demonstration
- #5838, WP-18B: provider-neutral multi-agent proof
- #5839, WP-19: birthday-to-governance handoff
- #5840, WP-20: demo matrix, AEE proof, and proof coverage
- #5844, WP-24: all ten launch articles, complete and review-ready
- #5845, WP-24A: the first ten podcast episodes, complete and review-ready

Serial gates:

- #5835 and #5836 follow the integrated birthday packet #5834.
- #5838 follows #5832, #5834, and #5836.
- #5840 follows #5836, #5837, #5838, and #5839.
- Final publication claims align with release truth only after #5843.

Early safe lane:

- After repository migration #5819, #5844 and #5845 may run in parallel in
  separate child worktrees. Each must produce ten complete review-ready works,
  not outlines, topic cards, or placeholders.

Other demo and handoff preparation may proceed separately as dependencies
stabilize, but no demo may claim unlanded Runtime, birthday, provider, or
governance behavior.

For each ready child: bind, create its goal, complete the deliverable, run real
proof or bounded source-grounded documentation validation, obtain exact-head
review, fix findings, and publish with `Closes #<child>`. Packet and proof links
that should be viewed independently must remain usable outside chat context.

Maintain watchers for waiting PRs and continue independent work. Preserve the
existing Observatory design unless an issue explicitly authorizes redesign.

Maintain:

- .csdlc/evidence/5854/activity.jsonl
- .csdlc/evidence/5854/sprint-review.md

Close #5854 only after real demos, handoff truth, proof coverage, ten articles,
and ten podcast packages are reviewed and all child outcomes are terminal.
```
