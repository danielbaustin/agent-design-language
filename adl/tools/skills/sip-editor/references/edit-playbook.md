# SIP Editor Playbook

Use this skill only for bounded `sip.md` editing.

Prefer this order:
1. source issue prompt
2. current `sip.md`
3. concrete lifecycle truth from repo state or caller findings

Check for:
- wrong branch/worktree state for the current phase
- stale placeholders or contradictory instructions
- target-file surfaces that no longer match the issue
- validation text that overstates what has happened
- pre-run vs run-bound confusion

Safe edits:
- normalize truthful branch/worktree state
- tighten inputs, targets, and validation plan
- remove placeholders and stale status text

Unsafe edits:
- creating execution state
- claiming finished work
- rewriting STP/SOR instead of handing off
