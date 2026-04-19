# Records-Hygiene Playbook

This playbook is the bounded operational algorithm for detecting and optionally
repairing ADL lifecycle record drift.

## Algorithm

1. Resolve the target issue context.
2. Load these surfaces when present:
   - source prompt
   - STP
   - SIP
   - SOR
   - worktree-local STP/SIP/SOR if bound
   - worktree path
3. Build a deterministic normalized evidence map:
   - issue number and slug
   - branch/worktree pairing
   - status states (`NOT_STARTED`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`)
   - PR metadata claims
   - run IDs and evidence IDs
   - integration wording (`worktree_only`, `pr_open`, `main_repo`)
4. Run drift checks:
   - stale completion state check
   - stale placeholder check
   - PR/worktree drift check
   - linkage drift check
   - identity drift check
   - integration truth check
5. Classify each finding:
   - `info` for low-impact cleanup hints
   - `warning` for non-blocking hygiene debt
   - `blocking` when execution/finish truth is materially wrong
   - `ambiguous` when human arbitration is needed
6. Apply safe fixes only when each fix:
   - has explicit deterministic replacement
   - is one of the supported safe fix types
   - is bounded to the resolved target surface
7. Emit contract artifact and follow-on recommendations.

## Supported Safe Fix Types

- Replace placeholder markers in bounded fields that are explicitly machine-determined.
- Drop duplicate placeholder entries already covered by a live value.
- Backfill run ID where all targets resolve to one canonical value.
- Align integration wording when worktree-bound status is now proven by branch/worktree inspection.

## Explicitly Unsupported Actions

- Adding or inferring issue intent, acceptance criteria, or implementation scope.
- Expanding into unbounded repo cleanup.
- Rewriting completed or historical records without explicit permission from a bounded target.
- Guessing issue intent from partial context.
- Modifying files outside the issue task bundle and worktree scope.
