# Burst Execution Playbook (`adl_pr_cycle`)

Use this playbook for sequential burst execution.

## Sequence

1. Generate a prioritized plan (max 8 issues).
2. Create child issues with `./swarm/tools/pr.sh new --no-start`.
3. Execute child issues one by one using `adl_pr_cycle` (`start -> codex -> finish -> report`).
4. Merge only green PRs.
5. Write a final summary under `.adl/reports/burst/<timestamp>/final_summary.md`.
   Use `swarm/tools/BURST_FINAL_SUMMARY_TEMPLATE.md`.

## Stop Conditions

- Stop on first hard failure in `finish`.
- Stop on policy violations.
- Stop when a human decision is required (scope, risk, or conflict resolution).

## Retry Policy

- Retry transient failures up to 2 times.
- Do not retry policy failures.
