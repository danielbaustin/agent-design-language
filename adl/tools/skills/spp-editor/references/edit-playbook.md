# SPP Editor Playbook

Use this skill only for bounded `spp.md` editing.

Prefer this order:
1. source issue prompt
2. current `spp.md`
3. linked `stp.md` or `sip.md` if available
4. concrete caller findings about stale plans or `codex_plan` drift

Check for:
- invalid `codex_plan` statuses outside `pending`, `in_progress`, and
  `completed`
- implementation steps marked complete without explicit execution evidence
- stale or weak source references
- missing dependencies, assumptions, risks, test strategy, or stop conditions
- missing review hooks or planning handoff notes
- placeholders or stale branch/worktree/execution claims

Safe edits:
- normalize `codex_plan` status values
- demote unsupported completed implementation steps to truthful planning state
- tighten source refs, dependencies, assumptions, risks, test strategy, stop
  conditions, and review hooks
- remove placeholders and stale execution wording

Unsafe edits:
- inventing implementation evidence
- claiming branch/worktree binding
- rewriting `STP`, `SIP`, or `SOR` instead of handing off
- widening issue scope
