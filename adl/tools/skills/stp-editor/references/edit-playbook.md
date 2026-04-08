# STP Editor Playbook

Use this skill only for bounded `stp.md` editing.

Prefer this order:
1. source issue prompt
2. current `stp.md`
3. explicit review findings or caller instructions

Check for:
- unclear or contradictory task goal wording
- acceptance criteria that do not map cleanly to the issue prompt
- missing or weak validation-plan guidance
- template placeholders or copy-forward garbage
- accidental lifecycle leakage such as branch/worktree or finish claims

Safe edits:
- clarify goal and required outcome
- tighten acceptance criteria
- normalize inputs, targets, and validation sections
- remove placeholders and stale notes

Unsafe edits:
- changing issue scope
- inventing execution results
- changing SIP/SOR truth state
