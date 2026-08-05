# v0.92 Sprint #5862 Session Prompt

Use this prompt to start the distributed Guardian implementation sprint after
the #5821 architecture and security gate is terminal.

```text
You own v0.92 sprint coordination issue #5862, WP-04-IMP Distributed Guardian
Implementation.

Read the repository root AGENTS.md first. Use only typed C-SDLC v2 lifecycle
tools. Never write tracked work on main. The umbrella coordinates child
sessions and terminal reconciliation only; it owns no product path and cannot
implement, review, merge, close, or self-attest a child on the child's behalf.

Startup contract:

1. Verify #5821 is terminal and its architecture/security design is ancestral
   to current main. Stop if that gate is absent, stale, or superseded.
2. Verify clean main, inspect active worktrees, and run typed doctor for #5862
   and every child #5863 through #5878.
3. Read issue #5862, `.csdlc/issues/5862/`,
   `.csdlc/prepared/issues/5862/design.md`, the #5821 child-wave design, and the
   canonical v0.92 issue wave.
4. Reacquire only the umbrella lifecycle/evidence claim. Every child acquires
   its own exact issue-local and product paths just in time.
5. Run `ruby .csdlc/prepared/issues/5862/validate-implementation-wave.rb
   --preflight` before scheduling any child.

Exact child wave:

- #5863 WP-04.01: membership and node identity
- #5864 WP-04.02: transport envelope and protocol
- #5865 WP-04.03: Runtime registration and manifest integration
- #5866 WP-04.04: authenticated peer ingress
- #5867 WP-04.05: quorum and membership transitions
- #5868 WP-04.06: partition behavior
- #5869 WP-04.07: fencing and stale-owner rejection
- #5870 WP-04.08: governed operation authorization
- #5871 WP-04.09: adversarial transport handling
- #5872 WP-04.10: distributed state transfer
- #5873 WP-04.11: migration and continuity
- #5874 WP-04.12: persistence and recovery
- #5875 WP-04.13: restart and rejoin
- #5876 WP-04.14: shutdown and drain
- #5877 WP-04.15: public API and WSS integration
- #5878 WP-04.16: final production, native, and adversarial integration proof

Scheduling contract:

- Follow the canonical dependency DAG exactly. Only dependency-ready children
  with disjoint exact owned paths may overlap.
- Bind each child in its own worktree, create its issue goal, implement the
  complete production outcome, retain real positive and negative proof, obtain
  exact-head independent review, fix all findings, and publish with
  `Closes #<child>`.
- Keep waiting PRs under watchers. A watcher has no merge or closeout authority.
- #5878 starts only after #5863 through #5877 are terminal. It must execute the
  production distributed validator and native receipt validator and recompute
  command-log, artifact, and digest evidence.
- Do not unblock WP-14 #5832 until #5862 terminal reconciliation verifies every
  child PR head, merge commit, closing relation, terminal receipt, ancestry,
  and #5878 integrated proof at the exact candidate head.

Maintain:

- `.csdlc/evidence/5862/activity.jsonl`
- `.csdlc/evidence/5862/child-terminal-matrix.json`
- `.csdlc/evidence/5862/sprint-review.md`

Before handoff, run the terminal form of
`ruby .csdlc/prepared/issues/5862/validate-implementation-wave.rb`. Preflight
success is preparation evidence only and cannot authorize WP-14 integration.
```
