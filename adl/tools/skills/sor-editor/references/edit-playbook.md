# SOR Editor Playbook

Use this skill only for bounded `sor.md` editing.

Prefer this order:
1. current `sor.md`
2. concrete execution evidence
3. finish or review findings

Check for:
- placeholder leakage such as enum menus or `ls <path>`
- overstated validation claims
- wrong integration wording for worktree-only or PR-open state
- stale branch/main assertions
- contradictions between summary and machine-readable fields

Safe edits:
- normalize integration wording to match reality
- replace placeholders with concrete supplied evidence
- tighten validation sections to only checks actually run
- align summary and machine-readable fields

Unsafe edits:
- inventing missing evidence
- claiming main merge before it happened
- publishing the PR instead of handing back to `pr-finish`
