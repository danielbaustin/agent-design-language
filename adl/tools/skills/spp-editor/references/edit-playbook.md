# SPP Editor Playbook

Use this skill only for bounded `spp.md` editing.

Prefer this order:
1. source issue prompt
2. current `spp.md`
3. linked `stp.md` or `sip.md` if available
4. concrete caller findings about stale plans or `codex_plan` drift

Check for:
- frontmatter drift away from the canonical manual sample shape
- missing `milestone_sprint` when the source issue is part of a sprinted
  milestone wave
- invalid `codex_plan` statuses outside `pending`, `in_progress`, and
  `completed`
- implementation steps marked complete without explicit execution evidence
- stale, weak, or missing source references to issue, STP, SIP, SOR, readiness
  docs, or dependencies
- missing `scope`, `constraints`, `confidence`, `proposed_steps`, or
  `execution_handoff`
- missing dependencies, assumptions, risks, test strategy, or stop conditions
- missing review hooks or planning handoff notes
- placeholders or stale branch/worktree/execution claims

Safe edits:
- restore the manual-sample schema shape and field names
- restore the manual-sample markdown heading order and section labels
- normalize `artifact_type` to `structured_planning_prompt`
- normalize `schema_version` to `0.1`
- normalize `codex_plan` status values
- demote unsupported completed implementation steps to truthful planning state
- tighten source refs, scope, dependencies, assumptions, risks, test strategy,
  stop conditions, and review hooks
- remove placeholders and stale execution wording

Unsafe edits:
- inventing implementation evidence
- dropping concrete issue-local planning detail in favor of generic prose
- claiming branch/worktree binding
- rewriting `STP`, `SIP`, or `SOR` instead of handing off
- widening issue scope
