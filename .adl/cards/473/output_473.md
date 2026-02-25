# ADL Output Card

Task ID: issue-0473
Run ID: issue-0473
Version: v0.7
Title: v0-7-wp-i-milestone-docs-bootstrap
Branch: codex/473-v0-7-wp-i-milestone-docs-bootstrap
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 (Codex)
- Provider: local
- Start Time: 2026-02-25
- End Time: 2026-02-25

## Summary
Recovered and applied the stash `wip: v0.7 milestone docs bootstrap (#473)` to the #473 branch, fixed stale worktree metadata blocking checkout, and prepared the branch for review PR.

## Artifacts produced
- Canonical v0.7 milestone docs:
  - `docs/milestones/v0.7/DESIGN_v0.7.md`
  - `docs/milestones/v0.7/WBS_v0.7.md`
  - `docs/milestones/v0.7/SPRINT_v0.7.md`
  - `docs/milestones/v0.7/DECISIONS_v0.7.md`
  - `docs/milestones/v0.7/MILESTONE_CHECKLIST_v0.7.md`
  - `docs/milestones/v0.7/RELEASE_PLAN_v0.7.md`
  - `docs/milestones/v0.7/RELEASE_NOTES_v0.7.md`
  - `docs/milestones/v0.7/SWARM_NAME_CHANGE_PLANNING_v0.7.md`
- Added docs scaffold content under `docs/milestones/v0.8/incubation/`:
  - `docs/milestones/v0.8/incubation/GODEL_AGENT.md`
  - `docs/milestones/v0.8/incubation/OBSMEM_BAYES.md`
- Removed v0.7 `*_TEMPLATE.md` files from `docs/milestones/v0.7/`.

## Actions taken
1. Located stash entry:
   - `stash@{0}: On main: wip: v0.7 milestone docs bootstrap (#473)`
2. Resolved stale worktree reference for branch `codex/473-v0-7-wp-i-milestone-docs-bootstrap`:
   - `git worktree prune`
3. Checked out #473 branch and applied stash:
   - `git checkout codex/473-v0-7-wp-i-milestone-docs-bootstrap`
   - `git stash apply stash@{0}`
4. Verified placeholder scan is clean:
   - `rg -n "\{\{.*\}\}" docs/milestones/v0.7 docs/milestones/v0.8`

## Validation
- Checks run:
  - `git stash show --name-status stash@{0}`
  - `rg -n "\{\{.*\}\}" docs/milestones/v0.7 docs/milestones/v0.8`
  - `find docs/milestones/v0.7 -maxdepth 2 -type f`
- Results:
  - Stash applied successfully to #473 branch.
  - No `{{...}}` placeholders remain in `docs/milestones/v0.7` or `docs/milestones/v0.8`.

## Decisions / Deviations
- Applied stash content exactly as requested (including v0.8 incubation docs present in stash).
- Focused this run on git recovery + branch application + PR creation.

## Follow-ups / Deferred work
- None.

## Addendum (User-requested follow-up)

Included additional changes in #473:
- Moved `.adl/*v0.3.md` files into `docs/milestones/v0.3/`:
  - `.adl/DECISIONS_v0.3.md` -> `docs/milestones/v0.3/DECISIONS_v0.3.md`
  - `.adl/DEMO_PLAYBOOK_v0.3.md` -> `docs/milestones/v0.3/DEMO_PLAYBOOK_v0.3.md`
  - `.adl/DESIGN_GOALS_v0.3.md` -> `docs/milestones/v0.3/DESIGN_GOALS_v0.3.md`
  - `.adl/RELEASE_CHECKLIST_v0.3.md` -> `docs/milestones/v0.3/RELEASE_CHECKLIST_v0.3.md`
  - `.adl/SPRINT_PLAN_v0.3.md` -> `docs/milestones/v0.3/SPRINT_PLAN_v0.3.md`
  - `.adl/WBS_v0.3.md` -> `docs/milestones/v0.3/WBS_v0.3.md`
- Included additional template update per request:
  - `.adl/templates/WBS_TEMPLATE.md`

These were added to the same #473 branch/PR to keep the docs migration bundled.
