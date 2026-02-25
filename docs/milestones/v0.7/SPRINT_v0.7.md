
# ADL v0.7 Sprint Plan

## Metadata
- Sprint: `v0.7-sprint-01`
- Milestone: `v0.7`
- Start date: 2026-02-24
- End date: 2026-03-02
- Owner: Daniel Austin

## Sprint Goal

Complete the v0.7 design pass and milestone-document bootstrap so v0.7 execution can proceed with a stable, agreed plan:
- Canonical milestone docs exist under `docs/milestones/v0.7/`.
- No placeholder tokens remain.
- WBS/decisions reflect the current issue taxonomy (EPIC vs WP vs task) and the newly created tail WPs.

## Planned Scope

- WP-01: Milestone docs bootstrap + doc set consistency (#473)
- Finalize the core milestone docs for v0.7:
  - `DESIGN_v0.7.md`
  - `WBS_v0.7.md`
  - `SPRINT_v0.7.md`
  - `DECISIONS_v0.7.md`
- Ensure the release tail is represented and parallelizable:
  - WP-13..WP-16 (#474–#477), EPIC-G (#478)
  - Rename work tracked as WP-12 (#336), EPIC-H (#479)

## Work Plan

| Order | Item | Issue | Owner | Status |
|---|---|---|---|---|
| 1 | Bootstrap canonical v0.7 milestone docs and generation/check script | #473 | Daniel + Codex.app | in-progress |
| 2 | De-mangle and finalize `DESIGN_v0.7.md` and record scope boundaries (learning train; ObsMem deferred to v0.8) | #473 | Daniel | done |
| 3 | Finalize WBS, including EPIC/WP/task taxonomy and mapping to current issue list | #473 | Daniel | in-progress |
| 4 | Finalize decisions log (learning phasing, ObsMem deferral, rename-last + compat window) | #473 | Daniel | done |
| 5 | Remove remaining placeholders across v0.7 docs and verify checks | #473 | Daniel + Codex.app | planned |
| 6 | Confirm tail WP issues exist and align titles (WP-13..WP-16) | #474–#477 | Daniel | done |

## Cadence Expectations

- Use issue cards (`input`/`output`) and `./swarm/tools/pr.sh` for execution work.
- Keep doc-only changes scoped; avoid runtime changes in WP-01.
- Keep scripts safe to run from `.adl/` or `tmp/` (never require repo root).

## Risks / Dependencies

- Dependency: GitHub permissions approvals for Codex.app in some environments.
  - Risk: Work stalls mid-PR due to permissions blocks.
  - Mitigation: Batch edits locally, minimize PR churn, and keep changes narrowly scoped per WP.

- Dependency: Template / placeholder hygiene.
  - Risk: curly braces, tokens or template remnants leak into canonical docs.
  - Mitigation: Enforce `rg -n "\{\{.*\}\}" docs/milestones/v0.7` as a hard gate.

## Demo / Review Plan

- Demo artifact: milestone docs present and coherent; `bootstrap_milestone_docs.sh --check` passes.
- Review date: 2026-03-02
- Sign-off owners: Daniel Austin

## Exit Criteria

- Canonical v0.7 milestone docs exist under `docs/milestones/v0.7/`.
- `rg -n "\{\{.*\}\}" docs/milestones/v0.7` returns no matches.
- WBS maps EPICs/WPs/tasks to the current v0.7 issue list, including EPIC-G/H and WPs 13–16.
- Decisions are recorded and consistent with the design.
