# v0.92 Sprint #5855 Session Prompt

Use this prompt to start the Runtime, Observatory, Polis, and Protocol sprint
session.

```text
You own v0.92 sprint coordination issue #5855, Runtime, Observatory, Polis, and
Protocol.

Read the repository root AGENTS.md first. Use only typed C-SDLC v2 lifecycle
tools. Never write tracked work on main, never use /private/tmp, and never let
network, SNTP, provider, certificate, logging, or Observatory failures take down
the Runtime path.

The umbrella coordinates child sessions only. Every child retains its own
worktree, claim, goal, implementation, proof, review, PR, and closeout authority.
Do not implement Runtime or Observatory code in the #5855 umbrella worktree.

Startup contract:

1. Verify WP-01 #5817 is merged and ancestral to current main.
2. Verify clean main and inspect active worktrees for Runtime/Observatory path
   collisions.
3. Read issue #5855, .csdlc/issues/5855/, both Sprint Execution Packets under
   .csdlc/prepared/issues/5855/, and the v0.92 issue wave.
4. Run typed doctor for #5855 and every child. Treat .csdlc issue records as
   canonical; do not recreate cards through a sunset .adl bundle path.
5. Prepare blocked children now, but do not cross a serial gate merely to keep
   the session busy.

Exact child wave:

- #5800: browser-trusted local Observatory HTTPS
- #5820, WP-03: Runtime launch and resilience consolidation
- #5795: governed local Gemma/MLX Shepherd MVP
- #5821, WP-04: distributed Guardian/polis runtime program
- #5832, WP-14: ACIP/A2A contract reconciliation and transport readiness
- #5837, WP-18A: Observatory and Unity consumer integration

The four Observatory issues #5800, #5820, #5795, and #5837 remain in this one
sprint. Do not split them into competing Observatory owners.

Serial gates:

- #5800 and #5820 establish the trusted local launch baseline.
- #5795 integrates only after #5800 and #5820 stabilize the path.
- #5837 integrates only after #5820, #5832, and its WP-18 dependency are ready.

Safe preparation and parallelism:

- #5821 architecture/security preparation and #5832 protocol preparation may
  proceed in separate child worktrees after Runtime ingress contracts stabilize.
- #5795 may prepare local-provider work before its final integration gate, but
  it may not redefine Runtime or Observatory contracts.

For every dependency-ready child: bind, create the child issue goal, implement
the complete production outcome, prove real positive and negative behavior,
review the exact head, fix all findings, and publish with `Closes #<child>`.
No fixture-only, demo-mode, substituted-provider, URL-only, or metadata-only
success receives completion credit.

Keep healthy waiting PRs under a watcher and continue only genuinely independent
lanes. Collapse any observed write collision to serial execution.

Maintain:

- .csdlc/evidence/5855/activity.jsonl
- .csdlc/evidence/5855/sprint-review.md

The final sprint review must assess the integrated Runtime/Observatory path,
resilience, authentication boundaries, logging/OTel truth, protocol compatibility,
and consumer behavior before #5855 closes.
```
